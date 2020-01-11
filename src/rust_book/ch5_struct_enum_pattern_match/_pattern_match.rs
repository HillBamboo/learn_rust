pub fn test_enum() {
    println!("XXD: it has a little bit ugly");
    {
        enum IpAddrKind {
            V4,
            V6,
        }

        struct IpAddr {
            kind : IpAddrKind,
            address : String,
        }

        let localhost = IpAddr{
            kind : IpAddrKind::V4,
            address : String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind : IpAddrKind::V6,
            address : String::from("::1"),
        };
    }

    println!("XXD: nice!");
    {
        enum IpAddr {
            V4(String),
            V6(String),
        }

        let localhost = IpAddr::V4(String::from("127.0.0.1"));
        let loopback =  IpAddr::V6(String::from("::1"));
    }

    {
        let x : i32 = 123;
        let y : Option<i32> = Some(5);
//        let sum = x + y; // compile error
    }
}
