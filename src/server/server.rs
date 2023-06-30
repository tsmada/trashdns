// server/server.rs

use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio::runtime::Runtime;
use std::sync::Arc;

use crate::server::request_handler::{DnsRequestHandler, RequestHandler};

pub struct Server {
    addr: SocketAddr,
}

impl Server {
    pub fn new(addr: SocketAddr) -> Self {
        Server { addr }
    }

    pub fn run(&self) {
        let rt = Runtime::new().unwrap();
        rt.block_on(self.run_server())
    }

    async fn run_server(&self) {
        let socket = UdpSocket::bind(self.addr).await.expect("Failed to bind address");
        println!("Listening on {}", socket.local_addr().unwrap());

        let socket = Arc::new(UdpSocket::bind(self.addr).await.expect("Failed to bind address"));
        let handler = DnsRequestHandler { socket: Arc::clone(&socket) };
        loop {
            let mut buf = vec![0u8; 512];
            let (amt, src) = socket.recv_from(&mut buf).await.expect("Failed to read data");
            buf.truncate(amt);

            let response = handler.handle_request(&buf).await;

            if !response.is_empty() {
                socket.send_to(&response, &src).await.expect("Failed to send data");
            }
        }
    }
}
