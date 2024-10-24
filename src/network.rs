use local_ip_address::local_ip;

use std::net::UdpSocket;
use std::time::Duration;

use std::io::{Error, ErrorKind};
use std::net::IpAddr;

use std::thread::{self};

const BROADCAST_ADDR: &str = "192.168.100.255:34254";

pub fn get_local_network_addr() -> IpAddr {
    let my_local_ip = local_ip().unwrap();

    println!("This is my local IP address: {:?}", my_local_ip);
    println!("Broadcast ip: {:?}", BROADCAST_ADDR);

    return my_local_ip;
}

fn ping_broadcast_channel(socket: UdpSocket) -> Result<(), Error> {
    match socket.send(&[0; 10]) {
        Ok(n) => {
            println!("{:?}", n);
            if n != [0; 10].len() {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Sent the wrong number of bytes",
                ));
            } else {
                // Do nothing because we sent the number of bytes we expected to send
                return Ok(());
            }
        }
        Err(e) => return Err(e),
    }
}

pub fn listen_to_broadcast_address() {
    let listen_thread = thread::spawn(|| {
        let socket: UdpSocket = UdpSocket::bind(BROADCAST_ADDR).unwrap();
        println!("Listen socket addr: {:?}", BROADCAST_ADDR);

        let connection_timeout = Some(Duration::new(5, 0));
        socket.set_broadcast(true).unwrap();
        socket.set_read_timeout(connection_timeout).unwrap();
        println!("Awaiting responses..."); // self.recv_buff is a [u8; 8092]
        let mut recv_buff = [0u8; 8092];
        while let Ok((n, addr)) = socket.recv_from(&mut recv_buff) {
            println!("{} bytes recieved from {:?}", n, addr);

            // Remaining code not directly relevant to the question
        }
    });

    listen_thread.join().unwrap();
    let _send_thread = thread::spawn(|| {
        let socket: UdpSocket = UdpSocket::bind(BROADCAST_ADDR).unwrap();
        socket.set_broadcast(true).unwrap();
        // println!("Connected on port {}", port);
        println!("Broadcast: {:?}", socket.broadcast());

        println!("Timeout: {:?}", socket.read_timeout());
        let _ = ping_broadcast_channel(socket);
    });
}

//get local network adress
//get mask
//ping every adress on subnet
//use threads
