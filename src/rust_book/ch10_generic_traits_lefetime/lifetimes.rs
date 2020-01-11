

pub fn test_longest() {
    {
        fn longest(s1 : &str, s2 : &str) -> String{
            let ret = if s1.len() > s2.len() {
                s1
            } else {
                s2
            };

            ret.to_string()
        }

        let string1 = String::from("long string is long");
        let result ;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
        }

        println!("The longest string is \"{}\"", result);
    }

    {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.')
            .next()
            .expect("Could not find a '.'");
        let i = ImportantExcerpt { part: first_sentence };
    }

    {
        fn longest<'a, 'b>(s1 : &'a str, s2 : &'b str) -> &'a str {
            s1
        }
    }
}

