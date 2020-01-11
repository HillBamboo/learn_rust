trait Summary {
    fn summarize(&self) {
        println!(
            "hello, summarize"
        );
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) {
        println!("Tweet")
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) {
        println!("NewsArticle")
    }
}

// TODO: how to fix it ? It occur compile error
//fn returns_summarizable(switch: bool) -> impl Summary {
//    if switch {
//        NewsArticle {
//            headline: String::from("Penguins win the Stanley Cup Championship!"),
//            location: String::from("Pittsburgh, PA, USA"),
//            author: String::from("Iceburgh"),
//            content: String::from("The Pittsburgh Penguins once again are the best
//            hockey team in the NHL."),
//        }
//    } else {
//        Tweet {
//            username: String::from("horse_ebooks"),
//            content: String::from("of course, as you probably already know, people"),
//            reply: false,
//            retweet: false,
//        }
//    }
//}