
pub fn test_match() {
    println!("\ntesting match... ");
    enum UsState {
        Apple,
        Banane,
        Watermelon,
    }

    let x : Option<UsState> = None;

    fn plus_one(x : Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    match none {
        Some(i32) => println!("i32"),
        None => println!("None"),
    };

    enum XXD {
        A,
        B,
        C,
        D,
    }


    // XXD : 怎样在编译器检查出没有match完整的分支，并中止编译？
    let xxd = Some(XXD::D);
    match xxd {
        Some(A) => println!("a"),
        _ => (),
    }

    let some_u8_value = Some(8u8);
    match some_u8_value {
        Some(8u8) => println!("three"),
        _ => (),
    }

    if let Some(8u8) = some_u8_value {
        println!("three");
    }

    println!("{}", some_u8_value.unwrap());
}