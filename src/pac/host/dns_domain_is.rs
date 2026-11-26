pub fn dns_domain_is(host: &str, domain: &str) -> bool {
    host.ends_with(domain)
}
