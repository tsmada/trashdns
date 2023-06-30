// src/queries/a.rs
use trust_dns_proto::op::Message;
use trust_dns_proto::rr::{Name, RData, Record, RecordType};
use std::net::Ipv4Addr;
use tokio::net::UdpSocket;
use std::error::Error;
use trust_dns_proto::serialize::binary::*;
use server::recursive_resolver::RecursiveResolver;
use std::str::FromStr;

pub async fn handle_a_query(
    query_message: &Message, 
    socket: &UdpSocket, 
    src: &std::net::SocketAddr, 
    resolver: &RecursiveResolver
) -> Result<(), Box<dyn Error>> {
    if let Some(query) = query_message.queries().first() {
        if query.query_type() == RecordType::A {
            let mut response_message = Message::new();
            response_message.set_id(query_message.id());
            response_message.set_op_code(query_message.op_code());
            response_message.set_message_type(trust_dns_proto::op::MessageType::Response);
            response_message.add_query(query.clone());

            let results = resolver.resolve(&query.name().to_ascii(), RecordType::A).await?;
            for result in results {
                let ip = Ipv4Addr::from_str(&result)?;
                let record = Record::from_rdata(
                    query.name().clone(),
                    3600,
                    RData::A(ip), 
                );
                response_message.add_answer(record);
            }

            let mut res_buf = Vec::new();
            let mut encoder = BinEncoder::new(&mut res_buf);
            response_message.emit(&mut encoder)?;

            socket.send_to(&res_buf, &src).await?;
        }
    }

    Ok(())
}
