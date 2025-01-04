use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;
use std::thread;

#[derive(Debug)]
struct Request {
    path: String,
    headers: HashMap<String, String>,
    _method: String,
}

struct Response {
    status_line: String,
    content_type: String,
    body: String,
}

impl Response {
    fn new(status_line: &str, content_type: &str, body: &str) -> Response {
        Response {
            status_line: status_line.to_string(),
            content_type: content_type.to_string(),
            body: body.to_string(),
        }
    }

    fn to_string(&self) -> String {
        format!(
            "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_line,
            self.content_type,
            self.body.len(),
            self.body
        )
    }
}

fn parse_request(reader: &mut BufReader<&mut std::net::TcpStream>) -> Request {
    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();
    let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
    
    let mut headers = HashMap::new();
    loop {
        let mut header_line = String::new();
        reader.read_line(&mut header_line).unwrap();
        
        // Empty line signals end of headers
        if header_line.trim().is_empty() {
            break;
        }
        
        // Parse header line
        if let Some((key, value)) = header_line.trim().split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }
    
    Request {
        _method: parts[0].to_string(),
        path: parts[1].to_string(),
        headers,
    }
}

fn handle_request(request: &Request, directory: &str) -> Response {
    if request.path == "/" {
        Response::new(
            "HTTP/1.1 200 OK",
            "text/html",
            "<h1>Hello, World!</h1>"
        )
    } else if request.path.starts_with("/echo/") {
        let echo_text = request.path.strip_prefix("/echo/").unwrap();
        Response::new(
            "HTTP/1.1 200 OK",
            "text/plain",
            echo_text
        )
    } else if request.path.starts_with("/files/") {
        // Add debug print to see what paths we're trying
        let file_path = format!("{}/{}", directory, request.path.strip_prefix("/files/").unwrap());
        println!("Trying to read file from: {}", file_path);  // Debug print
        
        let file_content = match std::fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(e) => {
                println!("Error reading file: {:?}", e);  // Debug print
                return Response::new(
                    "HTTP/1.1 404 Not Found",
                    "text/plain",
                    ""
                );
            }
        };
        Response::new(
            "HTTP/1.1 200 OK",
            "application/octet-stream",
            &file_content
        )
    } else if request.path.starts_with("/user-agent") {
        // Return the user agent in the response body
        let user_agent = request.headers.get("User-Agent").unwrap();
        Response::new(
            "HTTP/1.1 200 OK",
            "text/plain",
            user_agent
        )
    } else {
        Response::new(
            "HTTP/1.1 404 Not Found",
            "text/plain",
            ""
        )
    }
}

fn handle_connection(mut stream: TcpStream, directory: String) {
    let mut reader = BufReader::new(&mut stream);
    let request = parse_request(&mut reader);
    let response = handle_request(&request, &directory);
    let response_string = response.to_string();
    
    println!("Request: {:?}", request);
    println!("Response: {}", response_string);
    stream.write(response_string.as_bytes()).unwrap();
}

fn main() {
    // Handle `--directory` flag
    let directory = std::env::args()
        .collect::<Vec<String>>()
        .windows(2)
        .find(|args| args[0] == "--directory")
        .map(|args| args[1].clone())
        .unwrap_or(".".to_string());
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Listening on 127.0.0.1:4221");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let directory = directory.clone();
                thread::spawn(move || {
                    handle_connection(stream, directory);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
