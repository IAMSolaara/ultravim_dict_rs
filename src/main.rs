use std::{io::{Read, Write}, net::{Shutdown, TcpListener, TcpStream}, thread};

mod utils;

fn main() {
    println!("UltraVIM Dictionary Server starting up...");
    let listener = TcpListener::bind("0.0.0.0:27000").unwrap();
    println!("{:?}", listener);
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_connection(stream)
                });
            }
            Err(e) => {
                println!("Connection error: {}", e)
            }
        }
    }
    drop(listener);
}

fn handle_connection(mut stream: TcpStream) {
    let mut data = [0 as u8; 525];
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
    println!("DATA: `{:?}`", data);
}