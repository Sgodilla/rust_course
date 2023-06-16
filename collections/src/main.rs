fn main() {
    let a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // Vector macro
    let v2 = vec![1, 2, 3];

    // Scoped vector will be dropped after scope ends
    {
        let v3 = vec![1, 2, 3];
    }

    // Dangerous. Accessing index out of bounds will cause runtime error
    let third = &v[2];
    println!("The third element is {}", third);

    // Instead use get method and error handling
    match v.get(3) {
        Some(value) => println!("The fourth element is {}", value),
        None => println!("There is no fourth element."),
    }
}
