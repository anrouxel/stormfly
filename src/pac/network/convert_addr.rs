use std::{net::Ipv4Addr, str::FromStr};

pub fn convert_addr(ipchars: &str) -> u32 {
    let ip = Ipv4Addr::from_str(ipchars).unwrap();
    u32::from(ip)
}
