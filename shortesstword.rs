fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let input_string = "The quick brown fox jumps over the lazy dog";
    match shortest_word(input_string) {
        Some(shortest) => println!("Shortest word: {}", shortest),
        None => println!("No words found in the input string"),
    }
}
