//extern crate native_tls;
use native_tls_crate::{Identity, TlsAcceptor, TlsStream};
use std::fs::File;
use std::io::{Read};
//use std::net::{TcpListener, TcpStream};
use std::net::{TcpListener};
use std::sync::Arc;
use std::{thread::spawn};

use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};

fn main() {
    env_logger::init();
//    let server = TcpListener::bind("127.0.0.1:3012").unwrap();
    let mut file = File::open("domain.pfx").unwrap();
    let mut identity = vec![];
    file.read_to_end(&mut identity).unwrap();
    // the "" is password
    let identity = Identity::from_pkcs12(&identity, "").unwrap();

    let acceptor = TlsAcceptor::new(identity).unwrap();
    let acceptor = Arc::new(acceptor);

    let server = TcpListener::bind("0.0.0.0:3012").unwrap();

    for stream in server.incoming() {
        let acceptor = acceptor.clone();
        spawn(move || {
            let stream = acceptor.accept(stream.unwrap());

            let callback = |req: &Request, mut response: Response| {
                println!("Received a new ws handshake");
                println!("The request's path is: {}", req.uri().path());
                println!("The request's headers are:");
                for (ref header, _value) in req.headers() {
                    println!("* {}", header);
                }

                // Let's add an additional header to our response to the client.
                let headers = response.headers_mut();
                headers.append("MyCustomHeader", ":)".parse().unwrap());
                headers.append("SOME_TUNGSTENITE_HEADER", "header_value".parse().unwrap());

                Ok(response)
            };
            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();

            loop {
                let msg = websocket.read_message().unwrap();
                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}
