use std::net::{TcpListener, TcpStream, SocketAddrV4, Ipv4Addr};
use std::io::{Write, Read};
use crate::station::BikeRental;
use std::thread;
use std::sync::{Mutex, Arc, MutexGuard};
use std::time::Duration;

pub const DEFAULT_SERVER_PORT: u16 = 8080;
pub const DEFAULT_LOCAL_ADDRESS: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

struct ParsedValues<'a> {
    command: &'a str,
    person_id: &'a str,
    station_id: &'a str,
}

pub(crate) struct ServerOptions {
    port: Option<u16>,
    time_limit: Option<u8>,
}

impl ServerOptions {
    pub(crate) fn new_from_option(port: Option<String>, time_limit: Option<String>) -> Self {
        let port = match port {
            Some(v) => if let Ok(parsed) = v.parse::<u16>() { Some(parsed) } else { None },
            _ => None
        };

        let time_limit = match time_limit {
            Some(v) => if let Ok(parsed) = v.parse::<u8>() { Some(parsed) } else { None },
            _ => None
        };

        ServerOptions { port, time_limit }
    }
}

fn parse_data(data: &String) -> Result<ParsedValues, String> {
    let mut parts = data.split_whitespace();
    let first_arg = parts.next();
    let second_arg = parts.next();
    let third_arg = parts.next();

    match (first_arg, second_arg, third_arg) {
        (Some(command), Some(person_id), Some(station_id)) => {
            Ok(ParsedValues { command, person_id, station_id })
        }
        _ => Err(String::from("Some arguments are missing!"))
    }
}

fn execute(execute_parameters: ParsedValues, bike_rental: &mut MutexGuard<'_, BikeRental>) -> Result<String, String> {
    if execute_parameters.command == "rent" {
        return match bike_rental.register_rent(execute_parameters.person_id, execute_parameters.station_id) {
            Ok(r) => Ok(format!("Registered rent for {} at station {}.", r.id_person, r.id_station)),
            Err(e) => Err(e.to_string())
        };
    }

    if execute_parameters.command == "finish" {
        return match bike_rental.finish_rent(execute_parameters.person_id) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string())
        };
    }

    Err("Invalid Command sent.".to_string())
}

fn handle_client(mut stream: TcpStream, mut bike_rental: MutexGuard<'_, BikeRental>) {
    let mut buffer = [0; 1024];
    loop {
        let received_bytes = match stream.read(&mut buffer) {
            Ok(v) => v,
            Err(v) => {
                println!("Error reading data: {}", v);
                break;
            }
        };

        if received_bytes > 0 {
            let data = String::from_utf8(buffer[..received_bytes].to_vec()).unwrap();
            let remote_address = stream.peer_addr().unwrap();

            if data.eq("close") {
                println!("stream closed: {}", remote_address);
                break;
            }

            let write_result = match parse_data(&data) {
                Ok(parsed_values) => {
                    match execute(parsed_values, &mut bike_rental) {
                        Ok(result) => stream.write(result.as_bytes()),
                        Err(e) => stream.write(e.to_string().as_bytes())
                    }
                }
                Err(e) => stream.write(e.as_bytes())
            };

            match write_result {
                Ok(bytes) => println!("Replied to client {} -> {} bytes wrote.", remote_address, bytes),
                Err(e) => println!("Error writing to client {} -> {}", remote_address, e)
            }
        }
    }
}

pub(crate) fn run_server(options: ServerOptions) -> std::io::Result<()> {
    let server_port = match options.port {
        Some(port) => port,
        None => DEFAULT_SERVER_PORT
    };

    let time_limit: Option<Duration> = match options.time_limit {
        Some(limit) => Some(Duration::from_secs((limit * 60) as u64)),
        _ => None
    };

    let listener = TcpListener::bind(SocketAddrV4::new(DEFAULT_LOCAL_ADDRESS, server_port))?;
    // todo: add time limit parameter
    let bike_rental = BikeRental::new(Vec::new(), time_limit);
    let arc = Arc::new(Mutex::new(bike_rental));

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let reference = Arc::clone(&arc);
                thread::spawn(move || {
                    handle_client(stream, reference.lock().unwrap());
                });
            }
            Err(e) => println!("Client connection error: {}", e)
        }
    }
    Ok(())
}