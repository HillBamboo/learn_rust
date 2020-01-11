pub fn test_struct() {
    struct Rectangle {
        width : u32,
        height : u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        fn print_name() {
            println!("Rectangle");
        }
    }

    let r1 = Rectangle { width : 12, height : 33};
    Rectangle::print_name(); // XXD:  r1.print_name() is stopped by Rust compiler
}

