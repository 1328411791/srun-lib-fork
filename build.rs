use std::{env, net::Ipv4Addr};

fn main() {
    let auth_server_ip = std::env::var("AUTH_SERVER_IP").unwrap();
    println!("ENV AUTH_SERVER_IP = {auth_server_ip}");
    auth_server_ip
        .parse::<Ipv4Addr>()
        .expect("AUTH_SERVER_IP invalid");
}
