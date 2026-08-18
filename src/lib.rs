use std::io;

pub fn search(
    ignore_case: bool,
    query: &str,
    content_reader: impl Iterator<Item = io::Result<String>>,
) -> impl Iterator<Item = io::Result<(usize, String)>> {
    content_reader
        .enumerate()
        .filter_map(move |(idx, line)| match line {
            Ok(line) if matches(query, &line, ignore_case) => Some(Ok((idx + 1, line))),
            Ok(_) => None,
            Err(e) => Some(Err(e)),
        })
}

fn matches(query: &str, line: &str, ignore_case: bool) -> bool {
    match ignore_case {
        true => line.to_lowercase().contains(&query.to_lowercase()),
        false => line.contains(query),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        assert!(matches("duct", "safe, fast, productive.", false));
        assert!(!matches("duct", "Duct tape.", false));
    }

    #[test]
    fn case_insensitive() {
        assert!(matches("rUsT", "Rust:", true));
        assert!(matches("rUsT", "Trust me.", true));
    }
}
