#[allow(unused_imports)]
use std::net::TcpListener;
use std::io::Write;
use std::io::{BufRead, BufReader};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Listening on 127.0.0.1:4221");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // Respond with a 404 Not Found if the request is not GET /
                let mut request_line = String::new();
                let mut reader = BufReader::new(&mut stream);
                reader.read_line(&mut request_line).unwrap();

                let mut status_line = "HTTP/1.1 200 OK\r\n";
                let mut body = "<h1>Hello, World!</h1>";
                let mut headers = [
                    String::from("Content-Type: text/html\r\n"),
                    format!("Content-Length: {}\r\n", body.len()),
                    String::from("\r\n")  // Empty line to separate headers from body
                ].join("");

                if request_line != "GET / HTTP/1.1\r\n" {
                    status_line = "HTTP/1.1 404 Not Found\r\n";
                    body = "";
                    headers = [
                        String::from("Content-Type: text/plain\r\n"),
                        format!("Content-Length: {}\r\n", body.len()),
                        String::from("\r\n")  // Empty line to separate headers from body
                    ].join("");
                }

                let response = format!("{}{}{}", status_line, headers, body);
                println!("{}", response);
                stream.write(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
