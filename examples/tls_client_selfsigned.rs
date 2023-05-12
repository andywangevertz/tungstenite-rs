use tungstenite::{connect_with_config, Message, WebSocketConfig};
use url::Url;

fn main() {
    env_logger::init();

    let mut mywebcfg = WebSocketConfig::default();
    mywebcfg.ignore_trusted_cert = true;
    let (mut socket, response) =
        connect_with_config(Url::parse("wss://localhost:3012/socket").unwrap(), Some(mywebcfg), 3).expect("Can't connect");

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
