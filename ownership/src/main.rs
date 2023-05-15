fn main() {
    // ---- Ownership Rules ----
    // 1. Each value in Rust has a variable that's called its owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value will be dropped.

    // ---- Rules of References ----
    // 1. At any given time, you can have either one mutable reference or any number of immutable references
    // 2. References must always be valid

    { // s is not valid here, it's not yet delcared
        let s = String::from("hello"); // s is a valid string literal in this scope
        // s is still in scope
    } // scope has ended so s has been deallocated

    let x = 5;
    let y = x; // copies value of x into y
    // Both x and y are still valid
    println!("x: {x}, y: {y}");

    let s1 = String::from("hello");
    let s2 = s1;    // Move (not shallow copy), s1 has been invalidated

    // Below would be invalid because s1 has been moved to s2 
    // println!("{s1}");

    // Rather we must clone the value of s1 to s2 if we want both to be valid
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{s1}");

    // Passing a variable as an argument to function also takes ownership
    takes_ownership(s1);
    // Below would be once again invalid
    // println!("{s1}");

    // A function returning a heap-allocated value gives ownership
    let s1 = gives_ownership();
    // below println is now valid
    println!("{s1}");

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    // invalidates s2 and gives ownership to s3
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {s1}, s3 = {s3}");

    // Primitives do not have their ownership moved when passed as arguments
    makes_copy(x);
    println!("{x}");

    let s1 = String::from("hello");
    // pass reference to heap allocated variable, ie. borrowing
    let len = calculate_length(&s1);
    println!("length of {s1} is {len}");

    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");

    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // invalid, mutable references can only have one owner in a scope
    println!("{r1}"); // scope of r1 extends to last used reference
    let r2 = &mut s; // this is valid because r1's scope has ended
    println!("{r2}");
    
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s; // can have as many immutable references as desired
    // let r3 = &mut s; // invalid because cannot have mutable reference if immutable reference(s) exist previously  
    println!("{r1}");
    let r3 = &mut s; // this is valid because r1's scope has ended
    println!("{r3}");
    
    // below is a bad of getting the first word
    let mut s = String::from("hello world");
    let word = first_word(&s); // word contains index of first blank space (5)
    s.clear(); // now that s is cleared, word holds a valid but incorrect value (is 5, should be 0)

    // use String slices to fix this problem
    let mut s = String::from("hello world");
    let hello = &s[..5]; // returns a string slice/literal
    let world = &s[6..];
    let hello_world = &s[..]; 
    
    let word = first_word(&s);
    // s.clear() // produces error because word is an immutable references, clearing s would invalidate word
    println!("{word}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    return some_string;
}

fn takes_and_gives_back(a_string: String) -> String {
    return a_string;
}

// takes a reference to a String not the String itself, ie. borrowing
fn calculate_length(s: &String) -> usize {
    let length = s.len(); // references are immuatable by default
    return length; // local var s gets dropped at end of function
}

// passed references are immutable by default, must make them mutable to change them
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// invalid function, returns a borrowed value that is dropped in the function
// fn dangle() -> &String {
//     let s = String::from("hello");
//     return &s; // s is dropped at end of scope making reference to s invalid
// }

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // Returns the index of first blank space, therefore returning the first word
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}

fn sliced_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    // Returns the index of first blank space, therefore returning the first word
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

