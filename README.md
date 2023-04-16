# Experiment DHT 1

Experimental DHT, this first implementation was proposed by GPT-4.

This initial implement "runs" but does nothing useful as it's
simply two nodes with two separate hash tables. Thus you can
only `retrive()` information by know where it is. Not terribly
useful :)

## First question:

Implement a simple as possible DHT in rust

### Response:

A simple Distributed Hash Table (DHT) can be implemented in Rust using the Kademlia algorithm. For simplicity, we'll avoid using external crates and will provide a minimal example. Keep in mind that this is a simplified version and might not cover all edge cases.

Here's a minimal DHT implementation using Kademlia:
```
use std::collections::HashMap;
use std::net::SocketAddr;

type NodeId = u64;
type Key = String;
type Value = String;

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
```
This example shows how to create nodes and store/retrieve data locally. However, this is far from a complete DHT implementation as it doesn't include any network communication or Kademlia routing logic.

For a complete and production-ready implementation, consider using an existing crate like libp2p (https://crates.io/crates/libp2p) which provides the necessary networking and routing functionality.

### Run

```
wink@fwlaptop 23-04-15T18:10:44.488Z:~/prgs/rust/myrepos/exper_dht_1 (main)
$ cargo run
warning: fields `id` and `addr` are never read
  --> src/main.rs:9:5
   |
8  | struct Node {
   |        ---- fields in this struct
9  |     id: NodeId,
   |     ^^
10 |     addr: SocketAddr,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `exper_dht_1` (bin "exper_dht_1") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/exper_dht_1`
Node 1: Some("value1")
Node 2: Some("value2")
```

## Follow up question

You're right, the previous example is just a simple hash table and doesn't have any distributed functionality. Here's an updated version of the code that demonstrates a simplified Kademlia DHT. Please note that this is still a minimal example and not suitable for production use.

### Response

You're right, the previous example is just a simple hash table and doesn't have any distributed functionality. Here's an updated version of the code that demonstrates a simplified Kademlia DHT. Please note that this is still a minimal example and not suitable for production use.

```
use std::collections::{BTreeMap, HashMap};
use std::net::SocketAddr;

const BUCKET_SIZE: usize = 20;

type NodeId = u64;
type Key = String;
type Value = String;

#[derive(Debug)]
struct Node {
    id: NodeId,
    addr: SocketAddr,
    storage: HashMap<Key, Value>,
    buckets: [Vec<NodeId>; 64],
}

impl Node {
    fn new(id: NodeId, addr: SocketAddr) -> Self {
        Node {
            id,
            addr,
            storage: HashMap::new(),
            buckets: [Vec::new(); 64],
        }
    }

    fn distance(a: NodeId, b: NodeId) -> u64 {
        a ^ b
    }

    fn bucket_index(&self, other_id: NodeId) -> usize {
        (64 - self.distance(self.id, other_id).leading_zeros()) as usize
    }

    fn add_node(&mut self, other_id: NodeId) {
        let bucket_idx = self.bucket_index(other_id);
        let bucket = &mut self.buckets[bucket_idx];

        if !bucket.contains(&other_id) {
            if bucket.len() < BUCKET_SIZE {
                bucket.push(other_id);
            } else {
                // In a full implementation, we'd ping the oldest node and replace it if unresponsive
            }
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

    // Store and retrieve data locally
    node1.store("key1".to_string(), "value1".to_string());
    node2.store("key2".to_string(), "value2".to_string());

    // Add nodes to each other's buckets
    node1.add_node(node2.id);
    node2.add_node(node1.id);

    // This example still doesn't include network communication or Kademlia routing logic.
    // In practice, nodes would communicate with each other to find the closest node responsible for a key.
    // The storage and retrieval would happen on the closest nodes, rather than locally as in this example.
}
```
This version adds the concept of buckets to store NodeIds of other nodes in the network, simulating a simple Kademlia DHT. However, it still lacks networking and proper Kademlia routing functionality. For a complete and production-ready implementation, consider using the libp2p crate, as mentioned before.

# Several other Q's and finally getting there:

```
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::str::FromStr;

type Key = String;
type Value = String;

#[derive(Debug)]
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

    fn retrieve(&self, key: &Key) -> Option<&Value> {
        self.storage.get(key)
    }
}

fn handle_client(mut node: Node, mut stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut request = String::new();

    reader.read_line(&mut request).unwrap();
    let key = request.trim().strip_prefix("GET ").unwrap();

    if let Some(value) = node.retrieve(key) {
        writeln!(stream, "{}", value).unwrap();
    } else {
        writeln!(stream, "Key not found").unwrap();
    }
}

fn main() {
    let mut node1 = Node::new(1, "127.0.0.1:8000");
    let node2 = Node::new(2, "127.0.0.1:8001");

    node1.store("key1".to_string(), "value1".to_string());

    let node1_listener = TcpListener::bind(&node1.addr).unwrap();

    let node1_handle = thread::spawn(move || {
        for stream in node1_listener.incoming() {
            match stream {
                Ok(stream) => {
                    let node1_copy = node1.clone();
                    thread::spawn(move || handle_client(node1_copy, stream));
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

    // Close node1 listener
    drop(node1_listener);

    node1_handle.join().unwrap();
}
```

## Many more questions

It's finally got something kinda reasonable, we got a "self" contained
`Node` where each node is a separte thread and messages are sent between
nodes and from the "supervisor" which creates the nodes.

The current supervisor creates the nodes and then uses `add_peer` to add
the peers address to a particular. Currently every the supervisor has
every node knows the address of every other node.

I also asked GPT-4 to refactor so we now have a src/main.rs, src/node.rs,
src/lib.rs and tests/test.rs but no println logging.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
