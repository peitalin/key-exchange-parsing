
extern crate tokio;

use self::tokio::io;
use self::tokio::net::TcpListener;
use self::tokio::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};


pub fn serve() {
    // Bind the server's socket
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let tcp = TcpListener::bind(&addr).unwrap();

    let server = tcp.incoming().for_each(|socket| {
        println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());
        // Split up the read and write halves
        let (reader, writer) = socket.split();
        // Copy the data back to the client
        let conn = io::copy(reader, writer)
            // print what happened
            .map(|(n, _, _)| {
                println!("wrote {} bytes", n)
            })
            .map_err(|err| {
                println!("IO Error: {:?}", err)
            });
        // Spawn the future as a concurrent task
        tokio::spawn(conn);

        Ok(())
    })
    .map_err(|err| {
        println!("server error {:?}", err);
    });
    // Start the runtime and spin up the server
    tokio::run(server);
}
