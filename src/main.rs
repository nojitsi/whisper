mod network;

use die_exit::*;
use std::io::{ErrorKind, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;

const PORT_WIDTH: i32 = 70;

fn init_client() -> Result<TcpStream, &'static str> {
    //take 1 port

    for i in 0..PORT_WIDTH {
        let address = format!("127.0.0.1:577{:02}", i);
        println!("{}", address);
        let ping_client_r = TcpStream::connect(address.clone());

        if ping_client_r.is_ok() {
            println!("Client connected to {}", address);
            return Ok(ping_client_r.unwrap());
        }
    }

    return Err("Cannot find server on selected port range.");
}

fn init_server() -> Result<TcpListener, &'static str> {
    for i in 0..PORT_WIDTH {
        let address = format!("127.0.0.1:577{:02}", i);
        let ping_server_r = TcpListener::bind(address.clone());

        if ping_server_r.is_ok() {
            println!("Server started on {}", address);
            return Ok(ping_server_r.unwrap());
        }
    }

    return Err("Cannor start server on selected port range.");
}

fn handle_client_msg(mut stream: TcpStream) {
    let mut buf = [0 as u8; 256]; // using 50 byte buffer
    while match stream.read(&mut buf) {
        Ok(_) => {
            //panic on this unwrap during disconnection
            let client_msg = from_utf8(&buf).unwrap();
            println!("Client request msg is: {}", client_msg);
            let response = "...shh!";

            stream.write_all(response.as_bytes()).unwrap();

            let second_msg = "What do you want?";
            stream.write_all(second_msg.as_bytes()).unwrap();

            true
        }
        Err(e) if e.kind() == ErrorKind::ConnectionAborted => {
            println!("ErrorKind::ConnectionAborted");
            false
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn handle_server(listener: TcpListener) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client_msg(stream);
                });
            }
            Err(e) => {
                println!(
                    "Error during trying to establish connection with client: {}",
                    e
                );
                /* connection failed */
            }
        }
    }

    drop(listener);
}

//client need to know server initiated with same code
fn main() {
    let addr = network::get_local_network_addr();

    network::listen_to_broadcast_address();
    die!();
    let mut is_this_server = false;
    let mut ping_client: TcpStream = match init_client() {
        Ok(client) => client,
        Err(_e) => {
            println!("Local ping server not found");
            println!("Initiating new server");

            let server = match init_server() {
                Ok(server) => server,
                Err(e) => panic!("{}", e),
            };
            handle_server(server);
            is_this_server = true;

            init_client().unwrap()
        }
    };

    println!("Is this a server: {}", is_this_server);

    let msg = format!("Hay!");

    ping_client.write_all(msg.as_bytes()).unwrap();

    println!("Sent msg to server...");

    let mut read_buf = [0 as u8; 256]; // using 6 byte buffer

    while match ping_client.read(&mut read_buf) {
        Ok(_) => {
            let server_msg = from_utf8(&read_buf).unwrap();
            println!("Server msg is: {}", server_msg);

            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                ping_client.peer_addr().unwrap()
            );
            ping_client.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}

    println!("Done");
}
