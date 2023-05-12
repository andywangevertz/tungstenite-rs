use tungstenite::{connect, Message};
use url::Url;

/*
Connected to the server
Response HTTP code: 101 Switching Protocols
Response contains the following headers:
* connection
* upgrade
* sec-websocket-accept
* mycustomheader
* some_tungstenite_header
Received: Hello WebSocket
*/

fn main() {
    env_logger::init();
        // ideal case but url is not supporting sip://
        //connect(Url::parse("sip://1000:passwd@localhost:5060/register").unwrap()).expect("Can't connect");
    let (mut socket, response) =
        connect(Url::parse("ws://1000:passwd@localhost:5060/register").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    socket.write_message(Message::Text("Hello WebSocket".into())).unwrap();
    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }
    // socket.close(None);
}
