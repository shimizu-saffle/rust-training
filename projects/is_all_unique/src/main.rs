fn main() {
    println!("{}", is_all_unique("abc"));
}

fn is_all_unique(input: &str) -> bool {
    let mut chars_set = std::collections::HashSet::new();
    input.chars().all(|c| chars_set.insert(c))
}
