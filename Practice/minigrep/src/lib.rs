pub fn search<'a>(query: &str, file_content: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let mut results: Vec<&str> = vec![];

    if ignore_case {
        let query = query.to_lowercase();
        for line in file_content.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }
    } else {
        for line in file_content.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }
    }
    results
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
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, false)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "RuST";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, true))
    }
}
