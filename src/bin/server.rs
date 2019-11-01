use std::collections::VecDeque;
use std::io;
use std::net::UdpSocket;
struct UdpServer {
    socket: UdpSocket,
    queue: VecDeque<Query>,
}

use serde::de::{self, Visitor};
use serde_derive::Deserialize;
use serde_json::{Result, Value};

#[derive(Debug, Deserialize)]
enum QueryKind {
    Attack,
    Unknown(String),
}

// struct QueryKindVisitor;
// impl<'de> Visitor<'de> for QueryKindVisitor {
//     type Value = QueryKind;

//     fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
//     where
//         E: de::Error,
//     {
//         match v {
//             "attack" => Ok(QueryKind::Attack),
//             _ => Ok(QueryKind::Unknown(v.to_owned())),
//         }
//     }
// }

#[derive(Debug, Deserialize)]
struct Query {
    character_id: String,
    kind: QueryKind,
    payload: Value,
}

use std::str;
impl UdpServer {
    fn new(addr: &str) -> io::Result<Self> {
        let mut socket = UdpSocket::bind(addr)?;
        Ok(UdpServer {
            socket,
            queue: VecDeque::new(),
        })
    }

    fn start(&mut self) -> io::Result<()> {
        loop {
            let mut buf = [0; 1024];
            println!("receive...");
            let (num_of_bytes, src_addr) = self
                .socket
                .recv_from(&mut buf)
                .expect("didn't receive data");
            let filled_buf = &mut buf[..num_of_bytes];
            match str::from_utf8(filled_buf) {
                Ok(s) => {
                    eprintln!("{:?}", s);
                    let query: Result<Query> = serde_json::from_str(s);
                    println!("{:?}", query);
                    // TODO: dispatcherとの繋ぎ込み
                }
                Err(err) => {
                    eprintln!("{:?}", err);
                }
            }
        }
        Ok(())
    }
}

fn main() {
    let mut server = UdpServer::new("127.0.0.1:34254").unwrap();
    server.start();
    // TODO: queueを処理するスレッドを動かす
}
