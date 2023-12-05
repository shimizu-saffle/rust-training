fn main() {
    let numbers = vec![1,2,3,4,5];
    println!("{:?}", double_odd_numbers_in_vector(&numbers));
}

fn double_odd_numbers_in_vector(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().map(|&num| if num % 2 == 1 {num * 2} else {num}).collect()   
}
