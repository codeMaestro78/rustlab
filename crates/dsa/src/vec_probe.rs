fn main() {
    let mut v: Vec<i32> = Vec::new();
    println!("start           len={} cap={}", v.len(), v.capacity());
    for i in 0..10 {
        v.push(i);
        println!("after push {:>2}  len={} cap={}", i, v.len(), v.capacity());
    }
}
