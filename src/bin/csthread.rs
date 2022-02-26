#![feature(thread_is_running)]
#![feature(with_backlog)]

use std::io::Write;
use std::net::*;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

fn main() {
    let counter = Arc::new(AtomicU64::new(0));
    let listener = TcpListener::with_backlog("127.0.0.1:10123", 10_000).unwrap();
    let mut children = Vec::new();
    loop {
        let (mut socket, _addr) = listener.accept().unwrap();
        //eprintln!("new client: {:?}", addr);
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let count = counter.fetch_add(1, Ordering::SeqCst);
            write!(socket, "{}\r\n", count).unwrap();
            socket.flush().unwrap();
            drop(socket);
        });
        children.push(handle);
        if children.len() >= 100 {
            children = children
                .into_iter()
                .filter_map(|h| {
                    if h.is_running() {
                        Some(h)
                    } else {
                        let () = h.join().unwrap();
                        None
                    }
                })
                .collect();
        }
    }
}
