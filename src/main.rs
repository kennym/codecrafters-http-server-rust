#[allow(unused_imports)]
use std::net::TcpListener;
use std::io::Write;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Listening on 127.0.0.1:4221");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let status_line = "HTTP/1.1 200 OK\r\n";
                let body = "";
                let headers = [
                    String::from("Content-Type: text/plain\r\n"),
                    format!("Content-Length: {}\r\n", body.len()),
                    String::from("\r\n")  // Empty line to separate headers from body
                ].join("");

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
