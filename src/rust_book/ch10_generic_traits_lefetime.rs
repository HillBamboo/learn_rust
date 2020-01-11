
//fn largest(list : &[i32]) -> i32 {
//    let mut largest = list[0];
//    for &item in list.iter() {
//        if item > largest {
//            largest = item;
//        }
//    }
//
//    largest
//}

pub mod generic;
pub mod traits;
pub mod lifetimes;

use std::cmp::PartialOrd;

pub fn test() {
    test_generic();
}

fn largest<T : PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn test_largest() {
    let num_list = vec![34, 50, 25, 100, 65];
    let res = largest(&num_list);
    println!("The largest number is {}", res);

    let num_list = vec![55, 12, 66, 12];
    let res = largest(&num_list);
    println!("The largest number is {}", res);
}

pub fn test_generic() {
    struct Point<T, U> {
        x : T,
        y : U,
    }

    let x1 = Point {x : 5, y : 10.3};
    let x1 = Point {x : "5", y : 10.3};
    let x1 = Point {x : 5.123, y : 10.3};

//    impl<T, U> Point<T, U> {
//        fn mixup<V, W>(&self, other: Point<V, W>) -> Point<T, W> {
//            Point {
//                x: self.x, // XXD: how to implement the `Copy` trait for `Point<T, U>`???
//                y: other.y,
//            }
//        }
//    }
//
//    let p1 = Point { x: 5, y: 10.4 };
//    let p2 = Point { x: "Hello", y: 'c'};
//
//    let p3 = &p1.mixup(p2);
//
//    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

