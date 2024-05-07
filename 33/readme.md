
Single threaded webserver in Rust

In this example, we create a TcpListener that listens for incoming connections on port 8080. 
For each incoming connection, we spawn a new thread to handle the connection using the thread::spawn function.

In the handle_connection function, we read the incoming data from the client and respond with a simple HTML response.
This example serves a single HTML response for all requests, but you could modify it to serve different HTML files based on the request path.

Note that this is a very basic example and does not include error handling or security measures. 
For a more robust web server implementation, you may want to use a third-party crate or library that provides additional features and security measures.
