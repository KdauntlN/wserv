use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Command, Stdio};
use std::env;

fn supervisor_running() -> bool {
    TcpStream::connect("127.0.0.1:12345").is_ok()
}

pub fn start_server() {
    if !supervisor_running() {
        let current_path = env::current_exe().expect("Could not get current program path.");
        let mut _child = Command::new(current_path)
            .arg("supervisor")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Could not launch supervisor program");
    }

    let mut stream = TcpStream::connect("127.0.0.1:12345").expect("Could not connect to supervisor");
    stream.write_all("start-server".as_bytes()).unwrap();
    stream.flush().unwrap();

    let mut response = [0; 64];
    stream.read(&mut response).unwrap();
    println!("{}", String::from_utf8_lossy(&response));
}

pub fn shutdown() {
    if !supervisor_running() {
        return;
    }

    let mut stream = TcpStream::connect("127.0.0.1:12345").expect("Could not connect to supervisor");
    stream.write_all("shutdown".as_bytes()).unwrap();
    stream.flush().unwrap();

    let mut response = [0; 64];
    stream.read(&mut response).unwrap();
    println!("{}", String::from_utf8_lossy(&response));
}