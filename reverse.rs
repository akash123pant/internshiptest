fn reverse_string(input: &str) -> String {
    let reversed: String = input.chars().rev().collect();
    reversed
}

fn main() {
    let original_string = "Hello, world!";
    let reversed_string = reverse_string(original_string);
    println!("Original: {}", original_string);
    println!("Reversed: {}", reversed_string);
}
