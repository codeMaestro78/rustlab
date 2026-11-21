// Here we are learning The Slice Type
fn main() {
    let s = String::from("Hello World");
    let wordweneed = &s[0..5];
    println!("{wordweneed}");

    // Understanding String, str and &str
    // Type	                   Meaning
    // String	               Owned, growable string
    // str	                    String slice type
    // &str	                    Borrowed reference to a string slice

    //

    // A String:

    // let mut name = String::from("String");
    // name.push_str(" Editable");
    // println!("{name}");

    // Here, above name owns the string.
    // We can modify it if it is mutable:

    //  B) str
    // let name: &str = "Devarshi";
    let name = String::from("Devarshi");
    let reference: &str = &name;
    println!("Reference:-   {reference}");

    let s = String::from("Lalani devarshi");
    let word = first_word(&s);
    println!("First word without slice: {word}");

    let slice = first_word_slice(&s);
    println!("First word with slice: {slice}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

//
// creates a slice.
//
// It gives us a reference to a portion of the existing string.
//
// Notice three important things:
// - We did not create another String.
// - We did not copy "Hello".
// - We did not transfer ownership of s.
// We simply created a reference to a specific part of the existing data.
