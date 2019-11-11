#![allow(dead_code)]
use aworld_datagen::actions::character_action::CharacterAction;
use aworld_datagen::connection::Connection;
use aworld_datagen::context::Context;
use aworld_datagen::mappers::query_to_action;
use aworld_datagen::models::terrain::*;
use aworld_datagen::query::*;
use aworld_datagen::transactions::call_transaction_with;
use aworld_datagen::{dbg, err, log};
use std::collections::VecDeque;
use std::io;
use std::net::UdpSocket;
use std::str;
use std::sync::{Arc, RwLock};
use std::thread;

type ActionQueue = VecDeque<(String, CharacterAction)>;

struct UdpServer {
    socket: UdpSocket,
    queue: Arc<RwLock<ActionQueue>>,
}

impl UdpServer {
    fn new(addr: &str, queue: Arc<RwLock<ActionQueue>>) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        Ok(UdpServer { socket, queue })
    }

    fn start(&mut self) -> io::Result<()> {
        println!(
            "Aworld Data server v{} has started on {:?}",
            env!("CARGO_PKG_VERSION"),
            self.socket.local_addr().unwrap(),
        );
        loop {
            let mut buf = [0; 4096];
            dbg!("waiting datagram...");
            let (num_of_bytes, _src_addr) = self
                .socket
                .recv_from(&mut buf)
                .expect("[FATAL  ] didn't receive data");
            let filled_buf = &mut buf[..num_of_bytes];
            match str::from_utf8(filled_buf) {
                Ok(s) => {
                    log!("RECEIVE", "{:?}", s);
                    let query = serde_json::from_str::<Query>(s);
                    match query {
                        Ok(query) => match query_to_action(&query) {
                            Ok(action) => {
                                self.queue.write().unwrap().push_back((query.addr, action));
                            }
                            Err(err) => {
                                err!("Couldn't parse payload because expected {}", err);
                            }
                        },
                        Err(err) => {
                            err!("Couldn't parse query because {}", err);
                        }
                    }
                }
                Err(err) => {
                    err!("{:?}", err);
                }
            }
        }
    }
}

fn main() {
    let queue = Arc::new(RwLock::new(VecDeque::new()));
    let mut server = UdpServer::new("127.0.0.1:34254", queue.clone()).unwrap();
    thread::spawn(move || {
        let new_terrain = NewTerrain::default();
        let terrain = Terrain {
            id: 0,
            content: new_terrain.content,
            width: new_terrain.width,
            height: new_terrain.height,
        };
        let terrain_local = TerrainLocal::from(terrain);
        let mut context = Context::new(terrain_local);
        // TODO: データ投入
        loop {
            if let Some((ip, action)) = queue.write().unwrap().pop_front() {
                log!("ACTION", "{:?} from {:?}", action, ip);
                let conn = Connection { addr: ip };
                call_transaction_with(&conn, &mut context, action).unwrap_or_else(|e| {
                    err!("{}", e);
                });
            }
        }
    });
    server.start().unwrap();
}
