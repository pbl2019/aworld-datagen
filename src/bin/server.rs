#![allow(dead_code)]
use aworld_datagen::actions::Action;
use aworld_datagen::batches::hunger::*;
use aworld_datagen::connection::Connection;
use aworld_datagen::context::Context;
use aworld_datagen::mappers::query_to_action;
use aworld_datagen::models::item::*;
use aworld_datagen::models::{terrain::*, Entity};
use aworld_datagen::query::*;
use aworld_datagen::schedule::*;
use aworld_datagen::transactions::call_transaction_with;
use aworld_datagen::{dbg, err, log};
use std::collections::VecDeque;
use std::fmt;
use std::io;
use std::net::UdpSocket;
use std::str;
use std::sync::{Arc, RwLock};
use std::thread::{self, sleep};
use std::time::{Duration, Instant};

type ActionQueue = VecDeque<(String, i64, Action)>;

// TODO: クライアントへ送るjsonのシリアライズ
// TODO: もっとマシな方法を考える
struct CharacterView {
    pub character_id: u64,
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub items: Vec<u64>,
}

impl fmt::Debug for CharacterView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{{"character_id": {}, "x": {}, "y": {}, "angle": {}, "items": {:?}}}"#,
            self.character_id, self.x, self.y, self.angle, self.items
        )
    }
}

struct Point2D {
    pub x: i64,
    pub y: i64,
}

struct TerrainView {
    pub width: i32,
    pub height: i32,
    pub origin: Point2D,
    pub data: Vec<u8>,
}

impl fmt::Debug for TerrainView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{{"width": {}, "height": {}, "origin": {{"x": {}, "y": {}}}, "data": {:?}}}"#,
            self.width, self.height, self.origin.x, self.origin.y, self.data
        )
    }
}

struct ItemView {
    pub item_id: u64,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub is_dropped: bool,
}

impl fmt::Debug for ItemView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{{"item_id": {}, "name": {:?}, "x": {}, "y": {}, "is_dropped": {}}}"#,
            self.item_id, self.name, self.x, self.y, self.is_dropped
        )
    }
}
// ------------------------------------------ //

struct UdpSender {
    socket: UdpSocket,
}

impl UdpSender {
    fn new(addr: &str) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        Ok(Self { socket })
    }
    fn send(&self, data: &str, addr: &str) -> io::Result<usize> {
        self.socket.send_to(data.as_bytes(), addr)
    }
}

struct UdpReceiver {
    socket: UdpSocket,
    queue: Arc<RwLock<ActionQueue>>,
}

impl UdpReceiver {
    fn new(addr: &str, queue: Arc<RwLock<ActionQueue>>) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        Ok(Self { socket, queue })
    }

    fn start(&mut self) -> io::Result<()> {
        loop {
            let mut buf = [0; 8192];
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
                                self.queue
                                    .write()
                                    .unwrap()
                                    .push_back((query.addr, query.salt, action));
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
    let mut receiver = UdpReceiver::new("127.0.0.1:34254", queue.clone()).unwrap();
    let sender = UdpSender::new("127.0.0.1:34249").unwrap();

    println!(
        "Aworld Data server v{} has started on {:?}",
        env!("CARGO_PKG_VERSION"),
        receiver.socket.local_addr().unwrap(),
    );

    let new_terrain = NewTerrain::with_size(100, 100);
    let terrain = Terrain {
        id: 0,
        content: new_terrain.content,
        width: new_terrain.width,
        height: new_terrain.height,
    };
    let terrain_local = TerrainLocal::from(terrain);
    let context = Arc::new(RwLock::new(Context::new(terrain_local)));
    {
        let new_item = NewItem::default();
        let item = Item {
            id: 1,
            name: new_item.name,
            item_type: new_item.item_type,
            amount: new_item.amount,
        };
        let item_local = ItemLocal::from(item);
        item_local.x.write(100.0);
        item_local.y.write(100.0);
        context
            .write()
            .unwrap()
            .insert_entity(Entity::Item(Arc::new(item_local)));
    }
    let context2 = context.clone();
    let context3 = context.clone();

    thread::spawn(move || {
        // TODO: データ投入
        loop {
            if let Some((ip, salt, action)) = queue.write().unwrap().pop_front() {
                log!("ACTION", "{:?} from {}/{}", action, ip, salt);
                let conn = Connection { addr: ip, salt };
                call_transaction_with(&conn, &mut context.write().unwrap(), action).unwrap_or_else(
                    |e| {
                        err!("{}", e);
                    },
                );
            }
        }
    });
    let mut now = Instant::now();
    thread::spawn(move || loop {
        let mut mutated_entities;
        let connection_with_character_ids: Vec<(Connection, u64)>;
        {
            let mut lock = context2.write().unwrap();
            mutated_entities = lock.get_mutated_entities();
            connection_with_character_ids = lock
                .connection_to_character_id
                .iter()
                .map(|(conn, id)| (conn.clone(), *id))
                .collect();
        }
        mutated_entities.retain(|entity| match entity {
            Entity::Character(_) => true,
            Entity::Terrain(_) => true,
            Entity::Item(_) => true,
            _ => false,
        });
        if mutated_entities.is_empty() {
            continue;
        }
        let mutated_characters: Vec<CharacterView> = mutated_entities
            .iter()
            .filter(|entity| match entity {
                Entity::Character(_) => true,
                _ => false,
            })
            .map(|entity| match entity {
                Entity::Character(local) => CharacterView {
                    character_id: local.entity_id,
                    x: local.x.read(),
                    y: local.y.read(),
                    angle: local.angle.read(),
                    items: local.items.read(),
                },
                _ => panic!(),
            })
            .collect();
        let mutated_terrain: Vec<TerrainView> = mutated_entities
            .iter()
            .filter(|entity| match entity {
                Entity::Terrain(_) => true,
                _ => false,
            })
            .map(|entity| match entity {
                Entity::Terrain(local) => TerrainView {
                    width: local.model.width,
                    height: local.model.height,
                    origin: Point2D { x: 0, y: 0 },
                    data: local.raw.read(),
                },
                _ => panic!(),
            })
            .collect();
        let mutated_items: Vec<ItemView> = mutated_entities
            .iter()
            .filter(|entity| match entity {
                Entity::Item(_) => true,
                _ => false,
            })
            .map(|entity| match entity {
                Entity::Item(local) => ItemView {
                    item_id: local.entity_id,
                    name: local.model.name.clone(),
                    x: local.x.read(),
                    y: local.y.read(),
                    is_dropped: local.is_dropped.read(),
                },
                _ => panic!(),
            })
            .collect();
        let data = format!(
            r#""terrain": {}, "characters": {:?}, "items": {:?}"#,
            if mutated_terrain.len() > 0 {
                format!("{:?}", mutated_terrain[0])
            } else {
                "null".to_owned()
            },
            mutated_characters,
            mutated_items,
        );
        for (connection, character_id) in connection_with_character_ids.into_iter() {
            let buf = format!(r#"{{"character_id": {}, {}}}"#, character_id, data);
            sender.send(&buf, &connection.addr).unwrap_or_else(|e| {
                err!("{:?}", e);
                panic!(e)
            });
        }
        now = Instant::now();
        sleep(Duration::new(0, 5 * 1000 * 1000)); // NOTE: 5msスリープ
    });
    thread::spawn(move || {
        let mut now = Instant::now();
        let mut schedules = make_schedules();
        loop {
            {
                for schedule in &mut schedules {
                    let mut lock = context3.write().unwrap();
                    schedule.exec(&mut lock).map(|res| {
                        res.and_then(|mutations| {
                            lock.mark_mutations(mutations);
                            Ok(())
                        })
                        .unwrap();
                    });
                }
            }
        }
    });
    receiver.start().unwrap();
}
