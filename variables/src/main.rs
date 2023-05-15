fn main() {
    // Variables mutable by default, need to use mut keyword
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    
    // constants are always immutable, must have defined type
    const CONSTANT: u32 = 100000;
    println!("The value of CONSTANT is {CONSTANT}");

    // Shadowing (redefining) allows us to change types
    let y = 5;
    println!("The value of y is {y}");
    let y = "five";
    println!("The value of y is {y}");

    // Scalar Data Types:
    // Integers: i8, u8, i16, u16, ...., i128, u128, iarch, uarch
    // Floating-points
    // Booleans
    // Characters

    let a = 98_222;        // Decimal	
    let b = 0xff;         // Hex	
    let c = 0o77;         // Octal	
    let d = 0b1111_0000;  // Binary	 
    let e = b'A';         // Byte (u8 only)
    let f: u8 = 255; 
    
    // Operations

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // Booleans 
    let t = true;

    let f: bool = false; // with explicit type annotation

    // Characters use Unicode Scalar Value                
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = 0xF0;

    // Compound Types
    let tup: (&str, i32) = ("one hundred thousand", 100_000);
    let (str_name, int_count) = tup;
    let num_str = tup.1;

    // Arrays - fixed length
    let error_codes = [200, 400, 500];
    let not_found = error_codes[1];
    // let x = error_codes[3]; // invalid index

    let byte = [0; 8]; // [0,0,0,0,0,0,0,0]

    my_function();
    println!("11 + 13 = {}", my_sum(11, 13));
    
    // Control Flow
    let number = 5;

    // condition to be checked must be boolean
    if number < 10 {
        println!("first condition is true");
    } else if number < 22 {
        println!("second condition is true");
    } else {
        println!("condition was false");
    }
    
    // Ternary statement
    let condition = true;
    let number = if condition {5} else {6};

    // loops
    let mut counter = 0;
    let mut factorial = 1;
    // finds 10!
    let result = loop {
        counter += 1;
        factorial *= counter;
        if counter == 10 {
            break factorial; // returns counter
        }
    };
    println!("Result: {result}");
    
    // While loops
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
    
    // for looping through a collection
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("{element}");
    }
    for index in 1..4 {
        println!("{index}");
    }
}

// defining functions
fn my_function() {
    println!("Another function.");
}

// functions with parameter and return type, parameters need to have defined type
fn my_sum(x: i32, y: i32) -> i32 {
    return x + y;
}