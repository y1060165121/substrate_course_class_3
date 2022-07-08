use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    // try to connect to localhost:3333
    match TcpStream::connect("localhost:3333") {
        // if ok
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let msg = b"Hello!";
            // write msg to stream
            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            // try to read response into data
            match stream.read_exact(&mut data) {
                // if readable, see if echoed message matches original msg.
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                //if not readable, through failed to receive data and error message
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        // if not connected, print filed to connect
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
