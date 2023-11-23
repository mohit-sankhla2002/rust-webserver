use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use rust_webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Creating a pool of threads
    let pool: ThreadPool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // spawning a new thread every time we get a request
        pool.execute(|| {
            // pass the incoming TcpStream to the handle connection to handle the connection
            handle_connection(stream);
        });
        // handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    // routes here, then in the below if else condition: we can route the request to the controller using the routes matching given below.
    let get = b"GET / HTTP/1.1\r\n"; // the b as the prefix to the string makes sure that the string gets converted to byte slice
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("index.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        let contents = fs::read_to_string("index.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let response = "HTTP/1.1 405 Method Not Allowed\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
    }
}
