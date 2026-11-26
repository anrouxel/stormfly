pub fn is_plain_host_name(host: &str) -> bool {
    !host.contains('.') && !host.contains(':')
}
