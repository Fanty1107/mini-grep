pub fn search<'a>(query: &str, contents: &'a str) -> Vec<String> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.to_string());
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<String> {
    let mut results = Vec::new();
    let query_low = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query_low) {
            results.push(line.to_string());
        }
    }
    results
}
pub fn search_case_highlight<'a>(query: &str, contents: &'a str) -> Vec<String> {
    let mut text = Vec::new();
    let query_low = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query_low) {
            text.push(TextStyle::to_string(line, TextStyle::RED));
        } else {
            text.push(line.to_string());
        }
    }
    text
}

pub enum TextStyle {
    BOLD,
    ITALIC,
    UNDERLINE,

    GRAY,
    RED,
    GREEN,
    YELLOW,
    CYAN,
    VIOLET,
    BLUE,
}
impl TextStyle {
    pub fn get_prefix(&self) -> &str {
        match &self {
            Self::BOLD => "\x1b[1m",
            Self::ITALIC => "\x1b[3m",
            Self::UNDERLINE => "\x1b[4m",

            Self::GRAY => "\x1b[90m",
            Self::RED => "\x1b[91m",
            Self::GREEN => "\x1b[92m",
            Self::YELLOW => "\x1b[93m",
            Self::CYAN => "\x1b[94m",
            Self::VIOLET => "\x1b[95m",
            Self::BLUE => "\x1b[96m",
        }
    }
}

impl TextStyle {
    pub fn postfix() -> String {
        "\x1b[0m".to_string()
    }
    pub fn to_string(text: &str, style: TextStyle) -> String {
        format!("{}{}{}", style.get_prefix(), text, TextStyle::postfix())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
            search_case_insensitive(query, contents)
        )
    }
    #[test]
    fn case_highlight() {
        let cyan_str = TextStyle::to_string("Some text", TextStyle::CYAN);
        assert_eq!(cyan_str, "\x1b[94mSome text\x1b[0m");
    }
}
