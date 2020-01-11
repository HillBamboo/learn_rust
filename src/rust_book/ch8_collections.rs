
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
    for i in &v1 { i; }

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

pub fn test_tuple() {
//    let t1 = (1, 3, 4.34);
//    let v1 = vec![1, 2, 3, 4];
}

/// 事实上, String是Vec[u8]的包装类
pub fn test_string() {
    let d1 = "Initial contents";
    let s = d1.to_string();
    let s = "Initial contents".to_string();

    // .push_str() and .push() method
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2); // output : s2 is bar

    let c1 = 'f';
    s1.push(c1);
    println!("c1 is {}", c1); // output : c1 is f

    // formate macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", &s[1..2]);

}

pub fn test_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("green"), 50);

    // access item
    let score = scores.get(&String::from("blue"));
    println!("{}", score.unwrap());

    // update item
    // method 1
    scores.insert(String::from("blue"), 20);
    scores.insert(String::from("blue"), 50); // overwrite implicit

    // method2: more understandable
    scores.entry(String::from("blue")).or_insert(66); // .or_insert() return `&mut V`
}