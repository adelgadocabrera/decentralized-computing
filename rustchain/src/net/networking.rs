use std::net::SocketAddr;

pub fn get_addr(ip: &str, port: u16) -> SocketAddr {
    let addr: SocketAddr = format!("{}:{}", ip, port).parse().unwrap();
    return addr;
}
