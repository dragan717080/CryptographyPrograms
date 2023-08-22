fn main() {
    println!("Utils for strings");
}

pub fn find_repeated_substrings(input: &str, min_length: Option<usize>) -> Vec<String> {
    let min_len = min_length.unwrap_or(3);
    let mut repeated_substrings: Vec<String> = Vec::new();

    for len in min_len..=input.len() / 2 {
        for i in 0..=input.len() - len {
            let substring = &input[i..i + len];
            let rest_of_string = &input[i + len..];

            if rest_of_string.contains(substring) && !repeated_substrings.contains(&substring.to_string()) {
                repeated_substrings.push(substring.to_string());
            }
        }
    }

    repeated_substrings
}

pub fn get_alphabet() -> Vec<char> {
    (b'A'..=b'Z')
    .map(|ascii_value| char::from(ascii_value))
    .collect()
}
