use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::thread;

type Key = String;
type Value = String;

#[derive(Debug, Clone)]
struct Node {
    id: u64,
    addr: String,
    storage: HashMap<Key, Value>,
}

impl Node {
    fn new(id: u64, addr: &str) -> Self {
        Node {
            id,
            addr: addr.to_string(),
            storage: HashMap::new(),
        }
    }

    fn store(&mut self, key: Key, value: Value) {
        self.storage.insert(key, value);
    }

    fn retrieve(&self, key: &String) -> Option<&Value> {
        self.storage.get(key)
    }

    fn start(self) -> thread::JoinHandle<()> {
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
        let key = request.trim().strip_prefix("GET ").unwrap();

        if key == "terminate" {
            ClientStatus::Terminate
        } else {
            if let Some(value) = node.retrieve(&key.to_string()) {
                writeln!(stream, "{}", value).unwrap();
            } else {
                writeln!(stream, "Key not found").unwrap();
            }
            ClientStatus::Continue
        }
    }
}

#[derive(PartialEq, Eq)]
enum ClientStatus {
    Continue,
    Terminate,
}

fn main() {
    let node1_addr = "127.0.0.1:8000";
    let node2_addr = "127.0.0.1:8001";

    let mut node1 = Node::new(1, node1_addr);
    let node2 = Node::new(2, node2_addr);

    node1.store("key1".to_string(), "value1".to_string());

    let node1_handle = node1.start();

    // Connect node2 to node1 and request data
    let mut stream = TcpStream::connect(node1_addr).unwrap();
    writeln!(stream, "GET key1").unwrap();

    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();

    println!("Node2 received: {}", response.trim());

    // Close node1 listener
    let mut stream = TcpStream::connect(node1_addr).unwrap();
    writeln!(stream, "GET terminate").unwrap();

    node1_handle.join().unwrap();
}
