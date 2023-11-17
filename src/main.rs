use std::net::{ TcpListener, TcpStream };
use std::io::prelude::*;
use std::fs;

fn main() {
    let listener = 
        TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // pass the incoming TcpStream to the handle connection to handle the connection 
        handle_connection(stream);
    }
}

fn handle_connection(mut stream:TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n"; // the b as the prefix to the string makes sure that the string gets converted to byte slice 

    if buffer.starts_with(get) {
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
