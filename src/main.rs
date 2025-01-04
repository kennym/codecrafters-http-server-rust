#[allow(unused_imports)]
use std::net::TcpListener;
use std::io::Write;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Listening on 127.0.0.1:4221");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("returning 200");
                stream.write("HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n".as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
