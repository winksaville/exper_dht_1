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

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
