fn shortest_word(s: &str) -> Option<String> {
    s.split_whitespace()
        .min_by_key(|word| word.len())
        .map(|word| word.to_string())
}
