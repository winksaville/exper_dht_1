use std::collections::HashMap;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{BufRead, BufReader, Write};
use std::thread;

type Key = String;
type Value = String;

#[derive(Debug)]
struct Node {
    addr: String,
    storage: HashMap<Key, Value>,
}

impl Node {
    fn new(addr: &str) -> Self {
        Node {
            addr: addr.to_string(),
            storage: HashMap::new(),
        }
    }

    fn store(&mut self, key: Key, value: Value) {
        self.storage.insert(key, value);
    }

    fn retrieve(&self, key: &Key) -> Option<&Value> {
        self.storage.get(key)
    }
}

enum Action {
    Continue,
    Terminate,
}

fn main() {
    let mut node1 = Node::new("127.0.0.1:8000");
    let node2 = Node::new("127.0.0.1:8001");

    node1.store("key1".to_string(), "value1".to_string());

    let node1_listener = TcpListener::bind(&node1.addr).unwrap();

    // Spawn a thread to handle node1's incoming connections
    let node1_handle = thread::spawn(move || {
        for stream in node1_listener.incoming() {
            match stream {
                Ok(stream) => {
                    let storage_copy = node1.storage.clone();
                    let action = handle_client(stream, storage_copy);
                    if let Action::Terminate = action {
                        break;
                    }
                }
                Err(e) => {
                    println!("Error encountered while accepting connection: {}", e);
                }
            }
        }
    });

    // Connect node2 to node1 and request data
    let mut stream = TcpStream::connect(&node1.addr).unwrap();
    writeln!(stream, "GET key1").unwrap();

    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();

    println!("Node2 received: {}", response.trim());

    // Send termination command
    let mut stream = TcpStream::connect(&node1.addr).unwrap();
    writeln!(stream, "TERMINATE").unwrap();

    node1_handle.join().unwrap();
}

fn handle_client(mut stream: TcpStream, storage: HashMap<Key, Value>) -> Action {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut request = String::new();

    reader.read_line(&mut request).unwrap();
    if request.trim() == "TERMINATE" {
        stream.shutdown(Shutdown::Both).unwrap();
        return Action::Terminate;
    }

    let key = request.trim().strip_prefix("GET ").unwrap();

    if let Some(value) = storage.get(key) {
        writeln!(stream, "{}", value).unwrap();
    } else {
        writeln!(stream, "Key not found").unwrap();
    }

    Action::Continue
}
