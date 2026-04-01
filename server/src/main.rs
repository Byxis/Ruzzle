use std::net::SocketAddr;

mod room;
mod server;
use server::server;

fn main() {
    env_logger::init();
    println!("Usage: server [SERVER_PORT]");
    let args: Vec<String> = std::env::args().collect();

    let exec_type = &args[1];
    match exec_type.as_str() {
        "server" => {
            let server_addr: SocketAddr = format!("0.0.0.0:{}", args[2]).parse().unwrap();
            server(server_addr);
        }
        _ => {
            println!("Invalid argument, first one must be \"client\" or \"server\".");
        }
    }
}
