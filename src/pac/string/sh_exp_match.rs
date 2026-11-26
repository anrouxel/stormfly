use regex::Regex;

pub fn sh_exp_match(url: &str, pattern: &str) -> bool {
    let pattern = pattern
        .replace('.', r"\.")
        .replace('*', ".*")
        .replace('?', ".");

    let regex_pattern = format!("^{}$", pattern);
    match Regex::new(&regex_pattern) {
        Ok(re) => re.is_match(url),
        Err(_) => false,
    }
}
