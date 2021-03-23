use std::env;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect(); // collect args
    let config = Config::new(&args).unwrap_or_else(|err| { // parse args and check result, print error message
        println!("Problem parsing arguments: {}", err);
        process::exit(1); // exit running progress 
    });

    println!("Listening at: {}", config.addr); // print info
    let listener = TcpListener::bind(config.addr).unwrap(); // bind TcpListener to address

    for stream in listener.incoming(){ // loop through incomming connections
        let stream = stream.unwrap(); // unwrap stream to shadow

        println!("connection established"); // print info

        handle(stream); // handle stream
    }
}

// define a struct named Config to group arguments
struct Config {
    addr: String,
}

impl Config {
    // constructor for Config
    fn new(args: &[String]) -> Result<Config, &str>{
        if args.len() < 2{ // check args length
            return Err("not enough arguments");
        }

        let addr = args[1].clone(); // read args[1] and set to addr

        Ok(Config{addr}) // return Ok()
    }
}

fn handle(mut stream: TcpStream){ // accepts mutable TcpStream
    let mut buffer = [0;1024]; // setup a mutable buffer, initiate with 0, length 1024
    stream.read(&mut buffer).unwrap(); // read from stream
    println!("Request: {} ", String::from_utf8_lossy(&buffer[..])); // print incomming content, using String::from_utf8_lossy to convert byte to string

    let response = "How are you?\r\n"; // define response content

    stream.write(response.as_bytes()).unwrap(); // write to stream
    stream.flush().unwrap(); // flush stream
}
