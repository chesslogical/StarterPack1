


Multithreaded webserver in Rust


This code will generate an HTML page with a list of links to the other pages available on the server when the client requests the root path ("/"). The links are defined in a vector of tuples, where each tuple contains the path and text for a link. The links_html variable is created by mapping over the vector and generating an HTML list item for each link. Finally, the response variable is created by interpolating the links_html variable into an HTML template.

Note that this modified code assumes that there are HTML files called "about.html" and "contact.html" in the same directory as the server program. If these files do not exist, the server will return a "404 Not Found" error when the corresponding paths are requested.


If you want to add new HTML pages to the server, you'll need to create the corresponding HTML files and add code to the server program to serve those files when the corresponding paths are requested.

Here's an example of how you can add a new HTML page called "products.html" to the server:


Create a new file called "products.html" in the same directory as the server program. This file can contain any HTML code that you want.

Modify the handle_connection function in the server program to serve the "products.html" file when the "/products" path is requested. You can do this by adding a new arm to the match expression that checks for the "/products" path and calls the read_file function with the "products.html" file name. 



fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let path = request.split_whitespace().nth(1).unwrap_or("/");
    let content_type = match path {
        "/" => "text/html",
        "/about" => "text/html",
        "/contact" => "text/html",
        "/products" => "text/html",
        _ => "text/plain",
    };

    let response = match path {
        "/" => read_file("index.html"),
        "/about" => read_file("about.html"),
        "/contact" => read_file("contact.html"),
        "/products" => read_file("products.html"),
        _ => "404 Not Found".to_string(),
    };

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n{}",
        content_type, response
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}



With these changes, the server program will now serve the "products.html" file when the "/products" path is requested.

Note that you can add as many HTML files and corresponding paths to the server as you want by following these steps. Just create the new HTML file, and then modify the handle_connection function to serve the file when the corresponding path is requested.










