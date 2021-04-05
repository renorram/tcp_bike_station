use std::net::{TcpStream, SocketAddrV4};
use crate::server::{DEFAULT_SERVER_PORT, DEFAULT_LOCAL_ADDRESS};
use std::io::{Write, Read};
use std::str::FromStr;
use std::process::exit;

pub(crate) fn run_client(server_addr: Option<String>) -> std::io::Result<()> {
    let server_addr = if let Some(address) = server_addr {
        SocketAddrV4::from_str(address.as_str()).unwrap()
    } else {
        SocketAddrV4::new(DEFAULT_LOCAL_ADDRESS, DEFAULT_SERVER_PORT)
    };
    let mut stream = TcpStream::connect(server_addr)?;
    let mut input_buffer = String::new();
    let mut server_buffer = [0 as u8; 1024];

    loop {
        println!("Type a command:");
        if let Err(e) = std::io::stdin().read_line(&mut input_buffer) {
            println!("Error when capturing input: {}", e);
            continue;
        }

        if input_buffer.replace('\n', "").eq("exit") {
            stream.write("close".as_bytes())?;
            exit(0);
        }


        match stream.write(input_buffer.as_bytes()) {
            Err(e) => {
                println!("Error writing at stream socket: {}", e);
                input_buffer.clear();
                continue;
            }
            _ => {}
        }
        input_buffer.clear();

        let bytes_received = stream.read(&mut server_buffer)?;
        if bytes_received > 0 {
            println!("Server Response: {}", String::from_utf8(server_buffer[..bytes_received].to_vec()).unwrap());
        }
    }
}