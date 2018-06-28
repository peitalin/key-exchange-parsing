
extern crate tokio;
extern crate hyper;

// use self::tokio::prelude::*;

use std::net::{IpAddr, Ipv4Addr, SocketAddr, Incoming, TcpStream, TcpListener};
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;




pub fn serve(phrase: &str) {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer);

    let mut file = File::open("src/test.html").expect("File failed to open");
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes());
    stream.flush();
}




// pub fn tokio_serve(phrase: &str) {
//     // Bind the server's socket.
//     let addr = "127.0.0.1:3000".parse().unwrap();
//     let listener = tokio::net::TcpListener::bind(&addr)
//         .expect("unable to bind TCP listener");
//
//
//     // Pull out a stream of sockets for incoming connections
//     let server = listener.incoming().for_each(|socket| {
//         println!("Accepted socket: {:?}", socket.peer_addr().unwrap());
//         // Process socket here
//
//         let connection = tokio::io::write_all(socket, "<h1>Hello Tokio</h1>")
//             .then(|res| {
//                 println!("Hello Tokio, success={:?}", res.is_ok());
//                 Ok(())
//             });
//
//         // Spawn a new task that processes the socket:
//         tokio::spawn(connection);
//         Ok(())
//     })
//     .map_err(|err| {
//         println!("Accept error = {:?}", err);
//     });
//
//     // Start the Tokio runtime
//     tokio::run(server);
// }
//
//
