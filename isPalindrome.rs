impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let characters: Vec<char> = s
            .to_ascii_lowercase()
            .chars()
            .filter(|&x| x.is_ascii_alphanumeric())
            .collect();
        let length = characters.len();
        let first_half = &characters[..length / 2];
        let second_half = &characters[length - length / 2..];
        for (x, y) in first_half.iter().rev().zip(second_half) {
            if x != y {
                return false;
            }
        }
        true
    }
}
