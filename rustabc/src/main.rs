use std::net::{TcpListener, TcpStream};

fn main() {
    let listener: TcpListener = TcpListener::bind("0.0.0.0::8000").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
