fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut prefix = String::new();
    let first_string = &strings[0];

    'outer: for (i, c) in first_string.chars().enumerate() {
        for s in strings.iter().skip(1) {
            if i >= s.len() || s.chars().nth(i).unwrap() != c {
                break 'outer;
            }
        }
        prefix.push(c);
    }

    prefix
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    println!("Longest common prefix: {}", longest_common_prefix(&strings)); 

    let strings2 = vec![
        String::from("dog"),
        String::from("racecar"),
        String::from("car"),
    ];
    println!("Longest common prefix: {}", longest_common_prefix(&strings2)); 
}
