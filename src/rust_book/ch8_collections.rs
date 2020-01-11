
use std::collections::{HashMap, HashSet, VecDeque};

pub fn test_collections() {
    test_vec();
}

fn test_vec() {
    let v = vec![1, 2, 3];
//    let does_not_exist = &v[100];  // panic
    let does_not_exist = v.get(100);
//    let does_not_exist =  match does_not_exist {
//        Ok(num) => num,
//        Err(_) => continue,
//    };

//    println!("{}", does_not_exist);

// / compile error
//    let mut v = vec![1, 2, 3, 4, 5];
//    let first = &v[0];
//    v.push(6);
//    println!("The first element is: {}", first);

    let v1 = vec![100, 32, 56];
    for i in &v1 { i;}

    println!("Iterating over the values in a vector: ");
    let mut v1 = vec![100, 32, 56];
    print!("\nChange before : "); for i in &v1 {print!("{} ", i)};
    for i in &mut v1 { *i += 5; }
    print!("\nchanged after : ");
    for i in &v1 { print!("{} ", i)}

    println!("Using an Enum to Store Multiple Types: ");
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // if you don’t know the exhaustive set of types
    // the program will get at runtime to store in a vector,
    // the enum technique won’t work.
    // Instead, you can use a trait object
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn test_string() {
    let d1 = "Initial contents";
    let s = d1.to_string();
    let s = "Initial contents".to_string();
}