fn move_or_borrow() {
    //    fn foo1(s : &String) { s.push_str(", gagaga"); }
    fn foo2(s : &mut String ) {s.push_str("gagaag");}

    let mut sss = String::from("kalsfafa");
    println!("before : {}", sss);
    foo2(&mut sss);
    println!("after : {}", sss);

    let mut s = String::from("hello");
    {
        let r1 = &s;
        let r2 = &s;
        println!("{} and {}", r1, r2);
    }
    let r3 = &mut s;

    let r1 = &s;
    println!("{}", r1);
}

fn owership() {
    move_or_borrow();
}

fn borrow() {
    let ss = String::from("this is a second string");
    fn take_ownership(s : String) { println!("{}", s); };
    take_ownership(ss);
//    println!("{}", ss); // compile error because the ownership of ss is changed to s

    fn gives_ownership() -> String {
        let ret = String::from("some string val");
//        println!("gives_ownership : &ret = {:p}", ret); // TODO xxd: how to print out the address of a String type ??
        ret
    };
    let ss1 = gives_ownership();
//    println!("&ss1 = {:p}", ss1); // TODO xxd: how to print out the address of a String type ??
}

fn move_sematics_for_user_defined_type() {
    #[derive(Debug)]
    struct People (i32, i32);

    // 编译时知道类型的 = 的语义是 copy
    let x = 123;
    let y = x;
    println!("x = {}, y = {}", x, y);

    //// 编译时不知道大小的，= 的语义统统是 move
//    let s1 = String::from("hello, rust!");
//    let s2 = s1;
//    println!("s1 = {}, s2 = {}", s1, s2);

//    let p1 = People(12, 34);
//    let p2 = p1;
//    println!("p1 = {:?}, p2 = {:?}", p1, p2);
}


pub fn slice_type() {
    fn first_word(s : &String) -> usize {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' { return i; }
        }
        s.len()
    }

    fn logical_incorrect_code() {
        let mut s = String::from("hello world");
        let word = first_word(&s);
        println!("s = {}, word = {}", s, word);
        println!("Now s is cleared!!!");
        s.clear();
        println!("s = {}, word = {}", s, word);
    }

    println!(" ===> start : logical incorrect code: ");
    logical_incorrect_code();
    println!(" ===> end : logical incorrect code: ");

    fn improved_first_word(s : &String) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' { return &s[..i]; }
        }
        &s[..]
    }

    fn logical_correct_code() {
        let s = String::from("hello world");
        let word = improved_first_word(&s);
        println!("s = {}, word = {}", s, word);
        println!("trying clear s!!!");
//        s.clear(); // compile error
        println!("s = {}, word = {}", s, word);
    }

    println!(" ===> start : logical correct code: ");
    logical_correct_code();
    println!(" ===> end : logical correct code: ");
}

pub fn test_methods() {
    struct Point {
        x : i32,
        y : i32,
    }

    impl Point {
        fn show(&self) {
            println!("Point : x = {}, y = {}", &self.x, &self.y);
        }

        fn calc(self) -> i32 {
            self.x * self.y
        }
    }

    let p1 = Point { x: 12, y: 44 };
    p1.calc();
//    println!("{}", p1.x);
}