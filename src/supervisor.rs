use std::{io::{Read, Write}, net::TcpListener, process};

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
                let mut buf = [0; 64];
                let n = stream.read(&mut buf).unwrap();
                let message = str::from_utf8(&buf[..n]).unwrap().trim();

                match message {
                    "start-server" => stream.write_all("Server started".as_bytes()).unwrap(),
                    "shutdown" => {
                        stream.write_all("System shutting down".as_bytes()).unwrap();
                        process::exit(0);
                    }
                    _ => {
                        stream.write_all(format!("Invalid command: '{message}'").as_bytes()).unwrap();
                    }
                }

                stream.flush().unwrap();
            }
        }
    }
}