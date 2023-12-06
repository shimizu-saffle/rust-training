fn main() {
    let input_string = "hello world";
    let count_char = 'l';
    println!("Count: {}", count_specific_char_in_string(input_string, count_char));
}

fn count_specific_char_in_string(s: &str, char_to_count: char) -> usize {
    s.chars().filter(|&c| c == char_to_count).count()
}
