use std::{io::{Read, Write}, net::TcpListener};

pub struct Supervisor {
    listener: TcpListener,
}

impl Supervisor {
    pub fn new(addr: &str) -> Self {
        Self {
            listener: TcpListener::bind(addr).unwrap()
        }
    }

    pub fn run(&self) {
        loop {
            for stream in self.listener.incoming() {
                let mut stream = stream.unwrap();
                stream.write_all("Recieved connection\r\n".as_bytes()).unwrap();
                let mut request = String::new();
                stream.read_to_string(&mut request).unwrap();
                
                match &*request {
                    "start-server\r\n" => stream.write_all("Recieved request to start server\r\n".as_bytes()).unwrap(),
                    _ => stream.write_all("Invalid request\r\n".as_bytes()).unwrap(),
                }
                
            }
        }
    }
}