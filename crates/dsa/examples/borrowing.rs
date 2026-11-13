// The meaning of borrowing is that we can use a value without owning it.

fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
