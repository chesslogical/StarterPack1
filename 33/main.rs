use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n<!DOCTYPE html><html><head><title>Hello, Rust!</title></head><body><h1>Hello, Rust!</h1></body></html>";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
