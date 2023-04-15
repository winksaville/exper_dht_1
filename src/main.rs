use std::collections::HashMap;
use std::net::SocketAddr;

type NodeId = u64;
type Key = String;
type Value = String;

#[allow(unused)]
struct Node {
    id: NodeId,
    addr: SocketAddr,
    storage: HashMap<Key, Value>,
}

impl Node {
    fn new(id: NodeId, addr: SocketAddr) -> Self {
        Node {
            id,
            addr,
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

fn main() {
    // Create nodes
    let addr1 = "127.0.0.1:8000".parse().unwrap();
    let addr2 = "127.0.0.1:8001".parse().unwrap();
    let mut node1 = Node::new(1, addr1);
    let mut node2 = Node::new(2, addr2);

    // Store and retrieve data in the DHT
    node1.store("key1".to_string(), "value1".to_string());
    node2.store("key2".to_string(), "value2".to_string());

    println!("Node 1: {:?}", node1.retrieve(&"key1".to_string()));
    println!("Node 2: {:?}", node2.retrieve(&"key2".to_string()));
}

