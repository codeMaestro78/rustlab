fn main() {
    // let s = "Hello";

    // let s = String::from("Hello");
    // println!("{s}");


    // The kind of string can be mutated:
     let mut s = String::from("Hello");
     s.push_str(", world!");

     println!("{s}");


    //  #################      ownership transfer example         #################
    //      let a = String::from("a");
    //      let b = a;
    //      println!("{}", a);  // this will through an error as the ownership of a has been transferd to b. and a's memeory is free.

//  Now what if we want to use the two independent copies. we can use .clone()

     let a = String::from("This is a ");
     let b = a.clone();

     println!("a: {a}");
     println!("b: {b}");


    //  ########### WHAT IS COPY ? #################
    let x = 10;
    let y = x;

    println!("{x}");
    println!("{y}");

    // Both work because i32 implements Copy.

    // Common Copy types include:
    // - Integers (i32, u64)
    // - Floating-point numbers (f32, f64)
    // - Booleans (bool)
    // - Characters (char)
    // - Tuples containing only Copy types.

    // A String does not implement Copy, because copying its heap allocation requires more than a simple bitwise copy.

}



