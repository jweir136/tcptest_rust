use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn printout(mut stream: TcpStream) {
    let mut s = String::new();

    stream.read_to_string(&mut s);

    println!("{}", s);
}

fn server() {
    let listener = TcpListener::bind("localhost:3333").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) =>   {
                                println!("Connected!");
                                thread::spawn(move || {
                                    printout(stream)
                                });
                            },
            Err(err)         =>  {
                                panic!();
                            }
        }
    }

    drop(listener);
}

fn client(s: String) {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) =>   {
                                stream.write(s.as_bytes()).unwrap();
                            },
        Err(err) =>         {
                                panic!();
                            }
    };
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args[1] == "server".to_string() {
        server();
    } else if args[1] == "client".to_string() {
        client(args[2].to_string());
    } else {
        println!("Invalid argument");
    }
}
