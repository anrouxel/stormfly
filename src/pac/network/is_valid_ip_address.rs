use std::{net::Ipv4Addr, str::FromStr};

pub fn is_valid_ip_address(ipchars: &str) -> bool {
    Ipv4Addr::from_str(ipchars).is_ok()
}
