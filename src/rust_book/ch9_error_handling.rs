// two type of error:
// 1. recoverable error: using `Result<T, E>`
//    i.e open a unexist file
// 2. unrecoverable error: using `panic!` macro
//    i.e out of index

// panic! or not panic!
// panic! in this situation: Examples, Prototype Code, and Tests. `.unwrap()` and `.expected()` are two shortcut for panic!
// Not painc! when failure is expected. In this situation return a ``Result` type instead.
use std::io::ErrorKind;
use std::fs::File;
use std::fs;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

//    more ergonomic way
//    File::open("hello, txt")?
//        .read_to_string(&mut s)?;
//    Ok(s);

//    most common
//    fs::read_to_string("hello.txt")
}

pub fn test() {
    // 常规写法
    let f = match File::open("heex.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file : {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // 高级写法
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}