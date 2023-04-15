mod node;

use node::Node;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() {
    println!("main:+");

    let node1_addr = "127.0.0.1:8000";
    let node2_addr = "127.0.0.1:8001";

    let mut node1 = Node::new(1, node1_addr);
    let mut node2 = Node::new(2, node2_addr);

    node1.add_peer(node2.addr.clone());
    node2.add_peer(node1.addr.clone());

    println!("start node1");
    let handle1 = node1.start();
    println!("start node2");
    let handle2 = node2.start();

    println!(r#"client connects to node1_addr and issues "STORE key1 value1""#);
    let mut client = TcpStream::connect(node1_addr).unwrap();
    writeln!(client, "STORE key1 value1").unwrap();

    println!(r#"client issues "GET key1""#);
    let mut client2 = TcpStream::connect(node1_addr).unwrap();
    writeln!(client2, "GET key1").unwrap();
    writeln!(client2, "GET key1").unwrap();

    println!("client read response");
    let mut response = String::new();
    //BufReader::new(client2.try_clone().unwrap()).read_line(&mut response).unwrap();
    BufReader::new(client.try_clone().unwrap()).read_line(&mut response).unwrap();

    println!("Response: {}", response);

    // Terminate the listener threads
    println!("issue terminate to node1");
    let mut client3 = TcpStream::connect(node1_addr).unwrap();
    writeln!(client3, "GET terminate").unwrap();

    println!("issue terminate to node1");
    let mut client4 = TcpStream::connect(node2_addr).unwrap();
    writeln!(client4, "GET terminate").unwrap();

    println!("wait on handle1");
    handle1.join().unwrap();
    println!("wait on handle2");
    handle2.join().unwrap();

    println!("main:-1");
}
