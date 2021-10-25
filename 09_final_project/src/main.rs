// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 9. Final Project: Building a Concurrent Web Server with Async Rust
//         ^ https://rust-lang.github.io/async-book/09_example/00_intro.html
//    9.1. Running Asynchronous Code
//         ^ https://rust-lang.github.io/async-book/09_example/01_running_async_code.html
//    9.2. Handling Connections Concurrently
//         ^ https://rust-lang.github.io/async-book/09_example/02_handling_connections_concurrently.html
//    9.3. Testing the TCP Server
//         ^ https://rust-lang.github.io/async-book/09_example/03_tests.html
//
// -- Notes ---
// 1) `cargo run` - start the server
// 2) view the website at http://127.0.0.1:7878/
//
// -- Revisions --
// To view changes between 9. and 9.3 please check commit history:
// each chapter have its own related commit

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // Listen for incoming TCP connections on localhost port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Block forever, handling each request that arrives at this IP address
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Read the first 1024 bytes of data from the stream
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    // Respond with greetings or a 404,
    // depending on the data in the request
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    // Write response back to the stream,
    // and flush the stream to ensure the response is sent back to the client
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
