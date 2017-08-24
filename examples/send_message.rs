extern crate tokio_irc_client;
extern crate futures;
extern crate tokio_core;
extern crate pircolate;

use std::net::ToSocketAddrs;
use std::str::FromStr;
use tokio_core::reactor::Core;
use futures::future::Future;
use futures::Sink;
use futures::Stream;
use futures::stream;

use tokio_irc_client::Client;
use pircolate::message;

fn main() {
    // Create the event loop
    let mut ev = Core::new().unwrap();
    let handle = ev.handle();

    let mut server = "irc.freenode.org:6667".to_string();
    if let Ok(env_override) = std::env::var("IRC_SERVER") {
        server = env_override;
    }

    // Do a DNS query and get the first socket address for Freenode
    let addr = server.to_socket_addrs().unwrap().next().unwrap();

    // Create the client future and connect to the server
    // In order to connect we need to send a NICK message,
    // followed by a USER message
    let client = Client::new(addr)
        .connect(&handle)
        .and_then(|irc| {
            let connect_sequence = vec![
                message::client::nick("RustBot2"),
                message::client::user("RustBot2", "Example bot written in Rust"),
                message::client::join("#tokio-irc", None),
            ];

            irc.send_all(stream::iter(connect_sequence))
        })
        .and_then(|(irc, _)| {
            irc.send(
                message::client::priv_msg("#tokio-irc", "Hello World!").unwrap(),
            )
        })
        .and_then(|irc| {
            irc.send(
                message::client::priv_msg("#tokio-irc", "Goodbye world").unwrap(),
            )
        })
        .and_then(|irc| {
            irc.send(
                message::Message::from_str("PART #tokio-irc :are you still there\r\n").unwrap(),
            )
        })
        .and_then(|irc| irc.send(message::Message::from_str("QUIT").unwrap()))
        .and_then(|irc| irc.collect());

    ev.run(client).unwrap();
}
