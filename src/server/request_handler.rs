// request_handler.rs

use async_trait::async_trait;
use tokio::net::UdpSocket;
use trust_dns_proto::op::{Message, MessageType, OpCode, Query, ResponseCode};
use trust_dns_proto::rr::{DNSClass, RData, Record, RecordType};
use trust_dns_proto::serialize::binary::*;
use std::sync::Arc;

use crate::server::recursive_resolver;

#[async_trait]
pub trait RequestHandler {
    async fn handle_request(&self, buf: &[u8]) -> Vec<u8>;
}

pub struct DnsRequestHandler {
    pub socket: Arc<UdpSocket>,
}


#[async_trait]
impl RequestHandler for DnsRequestHandler {
    async fn handle_request(&self, buf: &[u8]) -> Vec<u8> {
        let msg = match Message::from_vec(buf) {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Failed to parse DNS message: {}", e);
                return vec![];
            }
        };

        let mut response = Message::new();
        response.set_id(msg.id());
        response.set_message_type(MessageType::Response);
        response.set_op_code(OpCode::Query);
        response.set_authoritative(false);
        response.set_truncated(false);
        response.set_recursion_desired(true);
        response.set_recursion_available(true);
        response.set_response_code(ResponseCode::NoError);
        let mut resolver = recursive_resolver::RecursiveResolver::new().await.unwrap();
        
        for query in msg.queries() {
            let query_name = query.name().to_utf8();
            let query_type = query.query_type();

            println!("Received query: {:?}", query);

            match query_type {
                RecordType::A => {
                    let result = resolver.resolve(&query_name, query_type).await;
                    match result {
                        Ok(ip_addrs) => {
                            for ip_addr in ip_addrs {
                                if let Ok(ip) = ip_addr.parse::<std::net::Ipv4Addr>() {
                                    let rdata = RData::A(ip);
                                    let record = Record::from_rdata(query.name().clone(), 3600, rdata);
                                    response.add_answer(record);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to resolve DNS query: {}", e);
                            response.set_response_code(ResponseCode::ServFail);
                        }
                    }
                }
                RecordType::AAAA => {
                    let result = resolver.resolve(&query_name, query_type).await;
                    match result {
                        Ok(ip_addrs) => {
                            for ip_addr in ip_addrs {
                                if let Ok(ip) = ip_addr.parse::<std::net::Ipv6Addr>() {
                                    let rdata = RData::AAAA(ip);
                                    let record = Record::from_rdata(query.name().clone(), 3600, rdata);
                                    response.add_answer(record);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to resolve DNS query: {}", e);
                            response.set_response_code(ResponseCode::ServFail);
                        }
                    }
                }
                _ => {
                    response.set_response_code(ResponseCode::NotImp);
                }
            }
            
        }

        let mut buf = Vec::new();
        let mut encoder = BinEncoder::new(&mut buf);

        match response.emit(&mut encoder) {
            Ok(_) => buf,
            Err(e) => {
                eprintln!("Failed to encode DNS response: {}", e);
                vec![]
            }
        }
    }
}
