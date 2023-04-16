use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

// Import the module from the main project
use my_dht::node::Node;

#[test]
fn two_node_test() {
    let node1_addr = "127.0.0.1:9000";
    let node2_addr = "127.0.0.1:9001";

    let mut node1 = Node::new(1, node1_addr);
    let mut node2 = Node::new(2, node2_addr);

    node1.add_peer(node2.addr.clone());
    node2.add_peer(node1.addr.clone());

    let handle1 = node1.start();
    let handle2 = node2.start();

    // Store a value in node1
    let mut stream1 = TcpStream::connect(node1_addr).unwrap();
    writeln!(stream1, "STORE key1 value1").unwrap();

    // Retrieve the value from node2
    let mut stream2 = TcpStream::connect(node2_addr).unwrap();
    writeln!(stream2, "GET key1").unwrap();

    let mut response = String::new();
    BufReader::new(stream2).read_line(&mut response).unwrap();

    assert_eq!("value1", response.trim());

    // Terminate node1
    let mut stream1 = TcpStream::connect(node1_addr).unwrap();
    writeln!(stream1, "GET terminate").unwrap();

    // Terminate node2
    let mut stream2 = TcpStream::connect(node2_addr).unwrap();
    writeln!(stream2, "GET terminate").unwrap();

    handle1.join().unwrap();
    handle2.join().unwrap();
}
