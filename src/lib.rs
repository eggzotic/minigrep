use std::io;

pub fn search<'a>(
    ignore_case: bool,
    query: &'a str,
    // contents: &'a str,
    content_reader: impl Iterator<Item = io::Result<String>>,
) -> impl Iterator<Item = String> {
    let query = if ignore_case {
        query.to_lowercase()
    } else {
        query.to_string()
    };

    content_reader
        .map(|line| line.unwrap())
        .filter(move |line| {
            let line = if ignore_case {
                line.to_lowercase()
            } else {
                line.to_string()
            };
            line.contains(&query)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text_as_iterator(text: &str) -> impl Iterator<Item = io::Result<String>> {
        text.split("\n").map(|s| io::Result::Ok(s.to_string()))
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(false, query, text_as_iterator(contents)).collect::<Vec<String>>()
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search(true, query, text_as_iterator(contents)).collect::<Vec<String>>()
        );
    }
}
