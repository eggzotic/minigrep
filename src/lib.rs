// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
pub fn search<'a>(
    ignore_case: bool,
    query: &str,
    contents: &'a str,
) -> impl Iterator<Item = &'a str> {
    let query = if ignore_case {
        query.to_lowercase()
    } else {
        query.to_string()
    };
    contents.lines().filter(move |&line| {
        let line = if ignore_case {
            line.to_lowercase()
        } else {
            line.to_string()
        };
        line.contains(&query)
    })
    // .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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
            search(false, query, contents).collect::<Vec<&str>>()
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
            search(true, query, contents).collect::<Vec<&str>>()
        );
    }
}
