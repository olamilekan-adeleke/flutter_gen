pub fn capitalize_first_word(s: &str) -> String {
    let mut chars = s.chars().peekable(); // Convert string to iterator over characters
    let mut result = String::new();

    // Check if the string is empty
    if chars.peek().is_none() {
        return result;
    }

    // Capitalize the first character
    let first_char = chars.next().unwrap_or(' '); // Get the first character, default to space if empty
    result.push(first_char.to_ascii_uppercase()); // Add capitalized first character to the result

    // Concatenate the rest of the original string starting from the second character
    result.push_str(&s[1..]);

    result
}
