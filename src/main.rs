// Copyright 2015 Justin Noah, All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![feature(core)]
#![feature(collections)]
#![feature(net)]
#![feature(std_misc)]

extern crate "rustc-serialize" as rustc_serialize;
extern crate rusqlite;

use std::net::UdpSocket;
use std::thread::Thread;

use rusqlite::SqliteConnection;

use handler::handle_response;

mod handler;
mod parse_packets;

// Initialize the database
fn init_db(path: &'static str) -> SqliteConnection {
    let conn = SqliteConnection::open(path).unwrap();
    conn.execute("
        CREATE TABLE IF NOT EXISTS torrent (
            info_hash   TEXT,
            ip          INTEGER,
            port        INTEGER,
            peer_id     TEXT,
            remaining   INTEGER,
            last_active INTEGER,
            PRIMARY KEY (info_hash, ip, port, peer_id)
        );",
        &[]
    ).unwrap();

    // And return the connection
    conn
}

fn main() {
    let database_path = "file::memory:?cache=shared";

    // Let's first initialize the database.
    let _ = init_db(database_path);
    let sock = UdpSocket::bind("0.0.0.0:6969").unwrap();

    loop {
        let mut buf = [0u8; 2048];
        let (amt, src) = sock.recv_from(&mut buf).unwrap();
        let tsock = sock.try_clone().unwrap();
        let mut b: Vec<u8> = buf.to_vec();
        b.truncate(amt);
        Thread::spawn(move|| {
            let conn = SqliteConnection::open(database_path).unwrap();
            handle_response(tsock, &src, b, &conn);
            let _ = conn.close();
        });
    }
}
