use std::{os::unix::net::UnixStream, io::{BufReader, Read}};

pub struct Client {
    stream: UnixStream
}

impl Client {
    pub fn new(stream: UnixStream) -> Client {
        Client {
            stream
        }
    }

    pub fn handle(&mut self){
        println!("Handling new client");
        
        let mut stream = BufReader::new(&self.stream);

        loop {
            let mut buf = String::new();

            stream.read_to_string(&mut buf).expect("Failed to read from stream");

            if buf.is_empty() {
                println!("Empty string, closing connection");
                break;
            }

            // parse message
            // 0 : command
            // 2 : comma seperated list of arguments(each argument is base64 encoded)

            let cmd: Vec<&str> = buf.split(" ").collect();

            
        }
    }
}