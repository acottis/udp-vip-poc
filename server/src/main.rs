use std::net::UdpSocket;

fn main() {
    let server = UdpSocket::bind("0.0.0.0:6969").unwrap();

    let hostname = std::process::Command::new("hostname")
        .output()
        .unwrap()
        .stdout;
    let hostname = std::str::from_utf8(&hostname).unwrap().trim();
    println!("INFO: Starting server, hostname: {hostname}");

    loop {
        let mut buf = [0u8; 1024 * 64];
        let (_, client) = server.recv_from(&mut buf).unwrap();
        println!("INFO: Client connected: {}", client);
        let msg = format!("Hello from {hostname}");
        server.send_to(msg.as_bytes(), client).unwrap();
    }
}
