use std::net::TcpListener;
use std::net::TcpStream;

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

fn handle_server(server: TcpListener) {
    for stream in server.incoming() {
        let _stream = stream.unwrap();

        println!("Client connected to server!");
    }
}

//client need to know server initiated with same code
fn main() {
    let mut is_this_server = false;
    let _ping_client: TcpStream = match init_client() {
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

    println!("{}", is_this_server);
}
