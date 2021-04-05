mod station;
mod server;
mod client;

use std::process::exit;
use crate::server::ServerOptions;

fn main() {
    let mut args = std::env::args().skip(1);
    let mode_arg = args.next();

    if let Some(mode) = mode_arg {
        if mode.eq("client") {
            if let Err(err) = client::run_client(args.next()) {
                println!("An error happened: {}", err.to_string());
                exit(1);
            }
        }

        if mode.eq("server") {
            let server_options = ServerOptions::new_from_option(args.next(), args.next());
            if let Err(err) = server::run_server(server_options) {
                println!("An error happened: {}", err.to_string());
                exit(1);
            }
        }

        println!("'{}' is not a valid run mode.", mode)
    }

    println!("You did not provide a run mode. modes available are 'client' and 'server'.");
    exit(1)
}
