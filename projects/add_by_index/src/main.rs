fn main() {
    let list_a = vec![1,2,3];
    let list_b = vec![4,5,6];
    let result = add_by_index(&list_a, &list_b);
    println!("{:?}", result);
}

fn add_by_index(list_a: &[i32], list_b: &[i32]) -> Vec<i32> {
    list_a.iter().zip(list_b.iter()).map(|(&a, &b)| a + b).collect()
}
