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
        println!("node{:?}:add_peer:+ peer_addr='{peer_addr}'", self.id);
        self.peers.push(peer_addr);
    }

    fn store(&mut self, key: Key, value: Value) {
        println!("node{:?}:store:+ key='{key}' value='{value}'", self.id);
        let r = self.storage.insert(key, value);
        println!("node{:?}:store:- r={r:?} len={}", self.id, self.storage.len());
    }

    fn retrieve(&self, key: &String) -> Option<&Value> {
        println!("node{:?}:retrieve:+ key='{key}' storeage.len={} storage={:?}", self.id, self.storage.len(), self.storage);
        let value = self.storage.get(key);
        println!("node{:?}:retrieve:- key='{key}' value='{value:?}'", self.id);
        value
    }

    pub fn start(mut self) -> thread::JoinHandle<()> {
        println!("node{:?}:start", self.id);
        let listener = TcpListener::bind(&self.addr).unwrap();

        thread::spawn(move || {
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
        })
    }

    fn handle_client(&mut self, mut stream: TcpStream) -> ClientStatus {
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut request = String::new();

        reader.read_line(&mut request).unwrap();
        println!("node{:?}:handle_client:+ request='{}'", self.id, request.trim());
        let tokens: Vec<&str> = request.trim().split(' ').collect();

        println!("node{:?}:handle_client: tokens[0]={}", self.id, tokens[0]);
        let result = match tokens[0] {
            "GET" => {
                let key = tokens[1];
                println!("node{:?}:handle_client: GET key='{key}'", self.id);

                if key == "terminate" {
                    println!("node{:?}:handle_client: key=terminate", self.id);
                    ClientStatus::Terminate
                } else {
                    if let Some(value) = self.retrieve(&key.to_string()) {
                        println!("node{:?}:handle_client: ket={key:?} = Some({value:?})", self.id);
                        writeln!(stream, "{}", value).unwrap();
                    } else {
                        println!("node{:?}:handle_client: not here, forward request", self.id);
                        // Forward the request to the next peer
                        for peer_addr in &self.peers {
                            println!("node{:?}:handle_client: not here, forward request to peer_addr={:?}", self.id, peer_addr);

                            let mut peer_stream = TcpStream::connect(peer_addr).unwrap();
                            writeln!(peer_stream, "GET {}", key).unwrap();

                            let mut peer_response = String::new();
                            let mut peer_reader = BufReader::new(peer_stream);
                            peer_reader.read_line(&mut peer_response).unwrap();

                            if peer_response.trim() != "Key not found" {
                                println!("node{:?}:handle_client: from peer_addr={:?} key={key} response={}", self.id, peer_addr, peer_response.trim());

                                writeln!(stream, "{}", peer_response.trim()).unwrap();
                                break;
                            }
                        }
                        println!("node{:?}:handle_client: key='{key}' not found", self.id);
                        writeln!(stream, "Key not found").unwrap();
                    }
                    ClientStatus::Continue
                }
            }
            "STORE" => {
                let key = tokens[1].to_string();
                let value = tokens[2].to_string();
                println!("node{:?}:handle_client: STORE key={key:?} value={value:?}", self.id);
                self.store(key, value);
                writeln!(stream, "Stored").unwrap();
                ClientStatus::Continue
            }
            _ => {
                println!("node{:?}:handle_client: invalid command={}", self.id, tokens[0]);
                writeln!(stream, "Invalid command").unwrap();
                ClientStatus::Continue
            }
        };
        println!("node{:?}:handle_client:- result={result:?}", self.id);

        result
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ClientStatus {
    Continue,
    Terminate,
}
   
