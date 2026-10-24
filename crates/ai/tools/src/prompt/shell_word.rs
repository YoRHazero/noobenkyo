/// `text` as one word of a shell command: as it is when the shell would
/// leave it alone, otherwise in single quotes.
pub fn shell_word(text: &str) -> String {
    let plain = !text.is_empty()
        && !text
            .chars()
            .any(|c| c.is_whitespace() || "'\"\\$`!*?[](){}<>|&;#~=%".contains(c));
    if plain {
        return text.to_owned();
    }
    // A single quote cannot appear inside single quotes: close, escape, reopen.
    format!("'{}'", text.replace('\'', r"'\''"))
}

#[cfg(test)]
mod tests {
    use super::shell_word;

    #[test]
    fn plain_words_stay_as_they_are() {
        assert_eq!(shell_word("猫"), "猫");
        assert_eq!(shell_word("食べる"), "食べる");
    }

    #[test]
    fn words_the_shell_would_split_or_expand_are_quoted() {
        assert_eq!(shell_word("お 茶"), "'お 茶'");
        assert_eq!(shell_word("$HOME"), "'$HOME'");
        assert_eq!(shell_word("it's"), r"'it'\''s'");
        assert_eq!(shell_word(""), "''");
    }
}
