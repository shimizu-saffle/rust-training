fn main() {
    let numbers = vec![1,2,3,4,5];
    let even_numbers = get_even_numbers(&numbers);
    println!("{:?}", even_numbers)
}

fn get_even_numbers(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().filter(|&&num| num % 2 == 0).cloned().collect()
}
