use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::thread;

type Key = String;
type Value = String;

#[derive(Debug, Clone)]
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

    fn store(&mut self, key: Key, value: Value) {
        self.storage.insert(key, value);
    }

    fn retrieve(&self, key: &String) -> Option<&Value> {
        self.storage.get(key)
    }

    pub fn start(self) -> thread::JoinHandle<()> {
        let listener = TcpListener::bind(&self.addr).unwrap();

        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let node_copy = self.clone();
                        let client_status = Self::handle_client(node_copy, stream);

                        if client_status == ClientStatus::Terminate {
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Error encountered while accepting connection: {}", e);
                    }
                }
            }
        })
    }

    fn handle_client(mut node: Node, mut stream: TcpStream) -> ClientStatus {
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
                    if let Some(value) = node.retrieve(&key.to_string()) {
                        writeln!(stream, "{}", value).unwrap();
                    } else {
                        // Forward the request to the next peer
                        for peer_addr in &node.peers {
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
                node.store(key, value);
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
   
