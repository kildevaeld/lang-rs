pub fn is_ascii_punctuation(input: &str) -> bool {
    for ch in input.chars() {
        if !ch.is_ascii_punctuation() {
            return false;
        }
    }

    true
}
