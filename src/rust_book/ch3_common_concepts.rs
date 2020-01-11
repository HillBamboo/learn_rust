pub fn variable() {
    let space = "   ";
    let space = space.len();
    println!("{}", space);

    let _a = [1, 2, 3, 4];
    let idx = 123;
    // println!("{}", a[idx]); // runtime error
    // println!("{}", a[6]); // compile error

    // 语法糖
    let a2 = [4; 6];
    let atuple = ("123", 2, 3);
}

pub fn control_flow() {
    if_expression();
    while_like_expression();
}

fn if_expression() {
    // if expression
    let condition = true;
    if condition {
        println!("true")
    } else {
        println!("false")
    };

    // let if statement
    let num = if condition {
        123
    } else {
        333
    };
    println!("{}", num);
}

fn while_like_expression() {
    // variant loop statement
    let mut idx= 0;
    while idx < 10 {
        idx += 1;
    }

    for number in (1..100).rev() {
        print!("{}!", number);
    }

    let mut cnt = 0;
    let cnt = loop {
        if cnt == 10 {
            break cnt * 2
        } else {
            cnt += 1
        }
    };
}
