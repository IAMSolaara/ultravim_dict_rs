use std::{
    collections::{hash_map::Entry, HashMap},
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

mod utils;

fn main() {
    println!("UltraVIM Dictionary Server starting up...");
    let listener = TcpListener::bind("0.0.0.0:27000").unwrap();
    println!("{:?}", listener);
    let dict_data: HashMap<String, Vec<String>> = HashMap::new();
    let data = Arc::new(Mutex::new(&dict_data));

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let data = Arc::clone(&data);
                thread::spawn(move || {
                    let mut my_data = data.lock().unwrap();

                    println!("handling conn from {:?}", stream.peer_addr());
                    let mut buffer = [0 as u8; 525];
                    let read_bytes = stream.read(&mut buffer).unwrap();
                    println!("Read {} bytes.", read_bytes);
                    let query_string = String::from_utf8(buffer[0..read_bytes].to_vec()).unwrap();
                    let query_string = query_string.trim();
                    println!("Query String: `{:?}`", query_string);
                    let query = utils::parse_query(&query_string);
                    match query {
                        Ok(query) => match my_data.entry(query.key) {
                            Entry::Vacant(e) => {}
                            Entry::Occupied(mut e) => {}
                        },
                        Err(e) => println!("Somting Wong!!!!!!: {}", e),
                    }

                    stream
                        .write(format!("TESTING. Data: {:?}\n", data).as_bytes())
                        .unwrap();
                    stream.flush().unwrap();
                });
            }
            Err(e) => {
                println!("Connection error: {}", e)
            }
        }
    }
    drop(listener);
}
