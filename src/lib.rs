use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self},
};


pub struct ThreadPool {
    workers: Vec<Worker>,
    // the sender here is a multiproducer single consumer channel
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, reciever) = mpsc::channel();

        let reciever = Arc::new(Mutex::new(reciever));

        let mut workers: Vec<_> = Vec::with_capacity(size);

        for id in 0..size {
            // create threads here on by one
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
// Worker is used to create a thread so that a thread can have a custom id
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = reciever
                .lock()
                .unwrap()
                .recv()
                .unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job, executing!", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate", id);
                }
            }
        });

        Worker { id: id, thread: Some(thread) }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all the workers!");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}