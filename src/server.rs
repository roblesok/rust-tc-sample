use clap::{App, Arg, ArgMatches};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn get_matches() -> ArgMatches<'static> {
    App::new("tcp-server")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .help("port the server runs on"),
        )
        .get_matches()
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0_u8; 32]; // 32 byte buff
    while match stream.read(&mut data) {
        Ok(size) => {
            // ECHO
            stream.write(&data[0..size]).unwrap();
            true
        }
        Err(_) => {
            println!(
                "Something went wrong! Finish {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let matches = get_matches();
    let port = matches.value_of("port").unwrap_or("3000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    println!("Server listening on port {}", port);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    drop(listener);
}
