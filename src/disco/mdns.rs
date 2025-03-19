use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::{timeout_at, Instant};

use crate::HueError;

/// Perform a mDNS oneshot "legacy" query to browse the available services with the given service_name
///
/// This function is a full reimplementation of mDNS because I haven't found a simple mDNS crate
/// that doesn't involve a daemon of some sort
pub async fn discover_mdns_sd(service_name: &str) -> Result<IpAddr, HueError> {
    // Note: this only binds on a single interface. If the device has multiple interfaces,
    // this won't perform the discovery on all interface
    let socket = UdpSocket::bind(SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(0, 0, 0, 0),
        0,
    )))
    .await
    .map_err(HueError::MdnsError)?;

    let dns_query_id = 4343; // Should use rand for it probably, but it should work like it is

    let mut dns_request_builder = dns_parser::Builder::new_query(dns_query_id, false);
    dns_request_builder.add_question(
        service_name,
        true, // Unicast because the Hue bridge will answer with Unicast DNS response if the request originates from another port than 5353
        dns_parser::QueryType::PTR, // DNS-SD browse queries use PTR query types, see RFC 6763 (4.1)
        dns_parser::QueryClass::IN, // Internet
    );
    let dns_request_bytes = dns_request_builder.build().unwrap();

    socket
        .send_to(
            &dns_request_bytes,
            &SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(224, 0, 0, 251), 5353)),
        )
        .await
        .map_err(HueError::MdnsError)?;

    let mut dns_response_bytes_buffer = [0_u8; 4096];
    let deadline = Instant::now() + Duration::from_secs(3);
    // This loop breaks when timeout_at returns an error
    loop {
        let (n_bytes, origin_addr) = timeout_at(deadline, socket.recv_from(&mut dns_response_bytes_buffer))
            .await
            .map_err(|_elapsed_error| {
                HueError::MdnsError(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "mDNS response was not received on time",
                ))
            })?
            .map_err(HueError::MdnsError)?;

        let src_ip = origin_addr.ip();
        let dns_response_bytes = &dns_response_bytes_buffer[0..n_bytes];
        if validate_response(dns_response_bytes, service_name, dns_query_id) {
            return Ok(src_ip);
        }
    }
}

/// Validates the response to make sure it matches the request
fn validate_response(dns_response_bytes: &[u8], service_name: &str, dns_query_id: u16) -> bool {
    fn validate_response_inner(dns_response_bytes: &[u8], service_name: &str, dns_query_id: u16) -> Option<bool> {
        let packet = dns_parser::Packet::parse(dns_response_bytes).ok()?;
        let first_question = packet.questions.first()?;
        Some(
            first_question.qtype == dns_parser::QueryType::PTR
                && first_question.qname.to_string() == service_name && packet.header.id == dns_query_id,
        )
    }
    validate_response_inner(dns_response_bytes, service_name, dns_query_id).unwrap_or(false)
}