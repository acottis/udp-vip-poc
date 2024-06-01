use std::{net::UdpSocket, str::from_utf8, thread::sleep, time::Duration};

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:6967").unwrap();
    socket
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    socket
        .set_write_timeout(Some(Duration::from_secs(1)))
        .unwrap();

    let addr = std::env::args().nth(1).expect(
        "Usage: Pass in the VIP address as the first argument\n\
        Example: ./client 172.19.1.20",
    );
    let target = format!("{addr}:6969");

    println!("INFO: Target IP is {target}");

    loop {
        let mut buf = [0u8; 1024 * 64];
        if socket.send_to(b"some data who cares", &target).is_err() {
            println!("ERROR: Error sending to {target}");
            sleep(Duration::from_secs(1));
            continue;
        };
        if socket.recv_from(&mut buf).is_err() {
            println!("ERROR: Error receiving from {target}");
            sleep(Duration::from_secs(1));
            continue;
        }

        let msg = from_utf8(&buf).unwrap();
        println!("INFO: received message: {msg}");
        sleep(Duration::from_secs(1));
    }
}
