pub fn local_host_or_domain_is(host: &str, hostdom: &str) -> bool {
    host == hostdom
        || hostdom
            .split_once('.')
            .is_some_and(|(prefix, _)| prefix == host)
}
