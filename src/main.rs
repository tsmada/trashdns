// src/main.rs
mod server;

use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use tokio::net::UdpSocket;
use trust_dns_proto::op::{Message, Query};
use trust_dns_proto::rr::{Name, RData, Record};
use trust_dns_proto::serialize::binary::*;
// use server::Server; TODO: wire this up
use server::recursive_resolver::RecursiveResolver;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting DNS server on 53");
    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:53").await?;
    let mut buf: [u8; 512] = [0; 512];
    let resolver: RecursiveResolver = RecursiveResolver::new().await?;

    loop {
        let (amt, src) = socket.recv_from(&mut buf).await?;
        let buf: &[u8] = &buf[..amt];
        let mut decoder: BinDecoder<'_> = BinDecoder::new(buf);
        let query: Message = Message::read(&mut decoder)?;

        if let Some(question) = query.queries().first().cloned() {
            let mut response: Message = Message::new();
            response.set_id(query.id());
            response.set_op_code(query.op_code());
            response.set_message_type(trust_dns_proto::op::MessageType::Response);
            response.add_query(Query::query(question.name().clone(), question.query_type()));

            if question.name().to_utf8() == "localhost." {
                println!("Responding to localhost query {}", question.query_type());
                let localhost_v4: Name = Name::from_str("localhost.")?;
                let answer_v4: Record =
                    Record::from_rdata(localhost_v4, 3600, RData::A(Ipv4Addr::new(127, 0, 0, 1)));
                response.add_answer(answer_v4);

                let localhost_v6: Name = Name::from_str("localhost.")?;
                let answer_v6: Record = Record::from_rdata(
                    localhost_v6,
                    3600,
                    RData::AAAA(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
                );
                response.add_answer(answer_v6);
            } else {
                println!("Resolving query {} {}", question.name(), question.query_type());
                let results: Vec<String> = resolver.resolve(&question.name().to_ascii(), question.query_type()).await?;
                for result in results {
                    let answer: Record = Record::from_rdata(question.name().clone(), 3600, RData::CNAME(Name::from_str(&result)?));
                    response.add_answer(answer);
                }
            }

            let mut res_buf: Vec<u8> = Vec::new();
            // compress output to reduce expose packet size
            let mut encoder: BinEncoder<'_> = BinEncoder::new(&mut res_buf);
            response.emit(&mut encoder)?;
            socket.send_to(&res_buf, &src).await?;
        }
    }
}
