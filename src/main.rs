mod node;

use node::Node;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() {
    let node1_addr = "127.0.0.1:8000";
    let node2_addr = "127.0.0.1:8001";

    let mut node1 = Node::new(1, node1_addr);
    let mut node2 = Node::new(2, node2_addr);

    node1.add_peer(node2.addr.clone());
    node2.add_peer(node1.addr.clone());

    let handle1 = node1.start();
    let handle2 = node2.start();

    let mut client = TcpStream::connect(node1_addr).unwrap();
    writeln!(client, "STORE key1 value1").unwrap();

    let mut client2 = TcpStream::connect(node1_addr).unwrap();
    writeln!(client2, "GET key1").unwrap();

    let mut response = String::new();
    BufReader::new(client2.try_clone().unwrap()).read_line(&mut response).unwrap();

    println!("Response: {}", response);

    // Terminate the listener threads
    let mut client3 = TcpStream::connect(node1_addr).unwrap();
    writeln!(client3, "GET terminate").unwrap();

    handle1.join().unwrap();
    handle2.join().unwrap();
}
