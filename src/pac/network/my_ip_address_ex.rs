pub fn my_ip_address_ex() -> String {
    let ip = local_ip_address::local_ipv6().unwrap();
    ip.to_string()
}
