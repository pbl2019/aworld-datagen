use aworld_datagen::mappers::query_to_action;
use aworld_datagen::query::*;
use std::collections::VecDeque;
use std::io;
use std::net::UdpSocket;
use std::str;

struct UdpServer {
    socket: UdpSocket,
    queue: VecDeque<Query>,
}

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
            let mut buf = [0; 4096];
            println!("receive...");
            let (num_of_bytes, src_addr) = self
                .socket
                .recv_from(&mut buf)
                .expect("didn't receive data");
            let filled_buf = &mut buf[..num_of_bytes];
            match str::from_utf8(filled_buf) {
                Ok(s) => {
                    eprintln!("{:?}", s);
                    let query = serde_json::from_str::<Query>(s);
                    println!("{:?}", query);
                    match query_to_action(query) {
                        Ok(action) => {}
                        Err(err) => {
                            eprintln!("{}", err);
                        }
                    }
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
