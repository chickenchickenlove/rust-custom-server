use std::io::{Read, Write};
use std::str::{from_utf8};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000")
        .unwrap();

    stream.write("Hello".as_bytes()).unwrap();
    let mut buffer = [0; 10];
    stream.read(&mut buffer).unwrap();

    println!(
        "Got response from server: {:?}",
        from_utf8(&buffer).unwrap()
    );
}
