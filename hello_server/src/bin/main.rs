extern crate clap;
extern crate hello_server;
use clap::{App, Arg};
use hello_server::ThreadPool;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process;
use std::thread;
use std::time::Duration;

fn main() {
    // setup CLI arg to specify port
    let matches = App::new("hello_server")
        .version("0.1.0")
        .author("Gustavo Blanco <gustavo.jr.blanco@gmail.com>")
        .about("simple web server")
        .arg(
            Arg::with_name("port")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("port web server will bind to"),
        )
        .get_matches();
    let port = matches.value_of("port").unwrap();

    // bing server to port
    let address = format!("127.0.0.1:{}", port);
    println!("listening on: http://{}", address);
    let listener = TcpListener::bind(address).unwrap();

    // create ThreadPool
    let pool = ThreadPool::new(4).unwrap_or_else(|err| {
        println!("error creating ThreadPool: {}", err);
        process::exit(1);
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    // read stream contents
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // hardcode Request types
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let file_path = "www/files/";

    // fire out what request gets
    let (status_line, filename) = if buffer.starts_with(get) {
        (
            "HTTP/1.1 200 OK\r\n\r\n",
            format!("{}hello.html", file_path),
        )
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        (
            "HTTP/1.1 200 OK\r\n\r\n",
            format!("{}hello.html", file_path),
        )
    } else {
        (
            "HTTP/1.1 404 NOT FOUND\r\n\r\n",
            format!("{}404.html", file_path),
        )
    };

    // open up file
    let mut file = File::open(filename).unwrap();

    // load file contents
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // build response
    let response = format!("{}{}", status_line, contents);

    // write response back into stream
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
