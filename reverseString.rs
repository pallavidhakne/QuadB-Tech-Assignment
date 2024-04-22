fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let input = "hello this is Pallavi";
    let reversed = reverse_string(input);
    println!("Original: {}, Reversed: {}", input, reversed);
}
