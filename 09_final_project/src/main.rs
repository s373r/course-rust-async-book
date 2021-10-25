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

use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;
use async_std::task::spawn;
use futures::stream::StreamExt;
use std::fs;
use std::time::Duration;

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    listener
        .incoming()
        // NOTE: use None as a limit we create N threads in a reactor
        //       to process incoming connections
        //       , where N is CPU count
        .for_each_concurrent(/* limit */ None, |stream| async move {
            let stream = stream.unwrap();
            // NOTE: spawn a task not a thread
            spawn(handle_connection(stream));
        })
        .await;
}

async fn handle_connection(mut stream: TcpStream) {
    // Read the first 1024 bytes of data from the stream
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // Respond with greetings or a 404,
    // depending on the data in the request
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    // Write response back to the stream,
    // and flush the stream to ensure the response is sent back to the client
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
