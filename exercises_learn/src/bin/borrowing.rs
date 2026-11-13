// //  The meaning of the borrowing is that we can use value without owning it
// fn main() {
//     let s1 = String::from("Hello");
//     let len = calculate_length(&s1);
//
//     println!("The length of s1 is : {}", len);
//
//     // Below is that example what happen if we try to modifiy something we are borrowing
//
//     //     let s = String::from("Hello");
//     //     change(&s);
//     //
//     //     println!("The new value is : {s}");
//
//     // Mutable references
//     //
//     let mut s = String::from("Hello");
//     mut_change(&mut s);
//
//     println!("The new of mutable references is : {s}");
//
//     // Mutable references have one big restriction: If you have a mutable reference to a value, you can have no other references to that value. This code that attempts to create two mutable references to s will fail:
//     //
//     //
//     //
//     // fn main() {
//     //     let mut score = 100;
//     //
//     //     let r1 = &mut score;
//     //     let r2 = &mut score;
//     //
//     //     println!("{r1}");
//     //     println!("{r2}");
//     // }
//
//     //
//     // But here we can fix this problem using the * called dereferencing the reference
//     //
//     let mut score = 100;
//     {
//         let r1 = &mut score;
//         *r1 += 10;
//
//         println!("First Update: {r1}");
//     }
//     // r1 is not longer in the scope
//
//     let r2 = &mut score;
//     *r2 += 10;
//     println!("Second update: {r2}");
//     println!("FInal updated score: {score}");
//
//     //  ****** Something important:-references dont necessarily last until the closing brace
//     //Rust uses concept called Non-Lexical Lifetimes(NLL)
//     //In simple words, a reference's borrowing period generally ends after its last use, not necessarily at the closing brace of the block.
//     // BELOW IS THE EXAMPLE:
//     let mut score1 = 100;
//
//     let r1 = &score1;
//     let r2 = &score1;
//
//     println!("The value of r1:- {r1} and r2:- {r2}");
//
//     let r3 = &mut score1;
//     *r3 += 50;
//     println!("The value of r3 : {r3}");
//
//     // ************************ Dangling References *******************************
//     // -> A dangling reference is a reference that pointing to a data that no longer exists.
//
//     let references = new_create_references();
//     println!("{references}");
// }
//
// // fn create_references() -> &String {
// //     let name = String::from("Dev");
// //     &name //-> this function's return type contains a borrowed value, but there is no value for it to be borrowed from
// // }
//
// // The solution for the dangle reference is that we can return the String
//
// fn new_create_references() -> String {
//     let s = String::from("Dev 2");
//     return s;
// }
//
// // fn change(s: &String) {
// //     s.push_str(", world!");
// // }   // as we are borrowing the value , we can modify the value, for modifying the value we have the below function.
//
// fn mut_change(s: &mut String) {
//     s.push_str(", Mutable references");
// }
//
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
//
// //
// // Three real reasons borrowing exists:
// //
// //   1. Reuse the value after the call — most common reason
// //   2. Avoid copying large data — if you have a Vec with 1 million elements, moving it into a function and back is wasteful. &Vec passes just a pointer
// //   3. Multiple readers at once — you can pass &s1 to 10 different functions simultaneously, you can't move s1 to 10 places

fn main() {
    // Read-only borrowing
    let name = String::from("Devarshi");
    print_name(&name);

    println!("The original main string: {}", &name);

    // Mutable borrowing
    let mut s1 = String::from("Hello");
    mut_update_string(&mut s1);
    println!("THe result is: {}", s1);

    // Multiple references

    let mut another = String::from("Hello multiple references");

    let r1 = &another;
    let r2 = &another;

    println!("This is r1:- {} and r2: {}", r1, r2);

    let r3 = &mut another;
    *r3 = String::from("New changes");
    println!("THe r3 is : {}", r3);
}

fn print_name(s: &String) {
    println!("The length of the string:- {}", s.len());
    println!("Original string: {}", s);
}

fn mut_update_string(s: &mut String) {
    s.push_str(", Rust");
}
