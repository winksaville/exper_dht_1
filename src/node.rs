use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::sync::mpsc;
use std::thread;

type Key = String;
type Value = String;

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Node {
    id: u64,
    pub addr: String,
    storage: HashMap<Key, Value>,
    peers: Vec<String>,
}

impl Node {
    pub fn new(id: u64, addr: &str) -> Self {
        Node {
            id,
            addr: addr.to_string(),
            storage: HashMap::new(),
            peers: vec![],
        }
    }

    pub fn add_peer(&mut self, peer_addr: String) {
        self.peers.push(peer_addr);
    }

    pub fn start(mut self) -> thread::JoinHandle<()> {
        let (started_tx, started_rx) = mpsc::channel::<()>();

        let listener = TcpListener::bind(&self.addr).unwrap();

        let join_handle = thread::spawn(move || {
            started_tx.send(()).unwrap();
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let client_status = self.handle_client(stream);

                        if client_status == ClientStatus::Terminate {
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Error encountered while accepting connection: {}", e);
                    }
                }
            }
        });

        started_rx.recv().unwrap();

        join_handle
    }

    fn handle_client(&mut self, mut stream: TcpStream) -> ClientStatus {
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut request = String::new();

        reader.read_line(&mut request).unwrap();
        let tokens: Vec<&str> = request.trim().split(' ').collect();

        match tokens[0] {
            "GET" => {
                let key = tokens[1];

                if key == "terminate" {
                    ClientStatus::Terminate
                } else {
                    if let Some(value) = self.storage.get(&key.to_string()) {
                        writeln!(stream, "{}", value).unwrap();
                    } else {
                        // Forward the request to the next peer
                        for peer_addr in &self.peers {
                            let mut peer_stream = TcpStream::connect(peer_addr).unwrap();
                            writeln!(peer_stream, "GET {}", key).unwrap();

                            let mut peer_response = String::new();
                            let mut peer_reader = BufReader::new(peer_stream);
                            peer_reader.read_line(&mut peer_response).unwrap();

                            if peer_response.trim() != "Key not found" {
                                writeln!(stream, "{}", peer_response.trim()).unwrap();
                                break;
                            }
                        }
                        writeln!(stream, "Key not found").unwrap();
                    }
                    ClientStatus::Continue
                }
            }
            "STORE" => {
                let key = tokens[1].to_string();
                let value = tokens[2].to_string();
                self.storage.insert(key, value);
                writeln!(stream, "Stored").unwrap();
                ClientStatus::Continue
            }
            _ => {
                writeln!(stream, "Invalid command").unwrap();
                ClientStatus::Continue
            }
        }
    }
}

#[derive(PartialEq, Eq)]
enum ClientStatus {
    Continue,
    Terminate,
}
