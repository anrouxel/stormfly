pub fn alert(message: &str) {
    tracing::warn!("{}", message)
}
