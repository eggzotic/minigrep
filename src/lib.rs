use std::io;

pub fn search(
    ignore_case: bool,
    query: &str,
    content_reader: impl Iterator<Item = io::Result<String>>,
) -> impl Iterator<Item = io::Result<String>> {
    let query = if ignore_case {
        query.to_lowercase()
    } else {
        query.to_string()
    };

    content_reader.filter_map(move |line| match line {
        Ok(line) => if ignore_case {
            line.to_lowercase().contains(&query)
        } else {
            line.contains(&query)
        }
        .then_some(Ok(line)),
        Err(e) => Some(Err(e)),
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
            search(false, query, text_as_iterator(contents)).collect::<io::Result<Vec<String>>>().unwrap()
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
            search(true, query, text_as_iterator(contents)).collect::<io::Result<Vec<String>>>().unwrap()
        );
    }
}
