pub fn dns_domain_levels(host: &str) -> usize {
    host.matches('.').count()
}
