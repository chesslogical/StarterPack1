use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
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

    let request = String::from_utf8_lossy(&buffer[..]);
    let path = request.split_whitespace().nth(1).unwrap_or("/");
    let content_type = match path {
        "/" => "text/html",
        "/about" => "text/html",
        "/contact" => "text/html",
        _ => "text/plain",
    };

    let response = match path {
        "/" => read_file("index.html"),
        "/about" => read_file("about.html"),
        "/contact" => read_file("contact.html"),
        _ => "404 Not Found".to_string(),
    };

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n{}",
        content_type, response
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => "404 Not Found".to_string(),
    }
}
