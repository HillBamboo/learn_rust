
#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test2() {
        assert_eq!(3 * 4, 12);
    }

    #[test]
    fn test3() {
        assert_eq!(
            1,
            1,
            "Greeting did not contain name, value was `{}`", 1
        );
    }

    #[test]
    #[should_panic(expected = "this function should be `panic!` !")]
    fn test4() {
//        panic!("this is the panic! message")
    }
}
