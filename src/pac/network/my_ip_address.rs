pub fn my_ip_address() -> String {
    let ip = local_ip_address::local_ip().unwrap();
    ip.to_string()
}
