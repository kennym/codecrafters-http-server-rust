use std::net::TcpListener;
use std::io::Write;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Request {
    path: String,
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

fn parse_request(request_line: &str) -> Request {
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    Request {
        _method: parts[0].to_string(),
        path: parts[1].to_string(),
    }
}

fn handle_request(request: &Request) -> Response {
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
    } else {
        Response::new(
            "HTTP/1.1 404 Not Found",
            "text/plain",
            ""
        )
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Listening on 127.0.0.1:4221");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut request_line = String::new();
                let mut reader = BufReader::new(&mut stream);
                reader.read_line(&mut request_line).unwrap();

                let request = parse_request(request_line.trim());
                let response = handle_request(&request);
                let response_string = response.to_string();
                
                println!("{}", response_string);
                stream.write(response_string.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
