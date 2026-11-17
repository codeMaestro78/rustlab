// Here we are learning The Slice Type
fn main(){
let s = String::from("Hello World");
let wordweneed = &s[0..5];
println!("{wordweneed}")
}
