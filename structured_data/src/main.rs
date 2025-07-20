use std::fmt::Debug;

/// Quiz this chapter is mostly about remembering the syntax
///     1. how to define a struct and how to use a struct
///     2. how to use specific value from struct
///     3. how to define a function with params and return values
///     4. what is statements and what is expressions
///     5: loop, while and for syntax to write loops
///
///  Things worthy to be mentioned
///     the expression in if statement MUST be a bool
///     multiple branch must have the same type, cuz rust is a static type language

fn main() {

    // how to define a struct
    #[derive(Debug)]
    struct InnerStruct {
        metadata: String,
        _created_at: u64,
        _active: bool,
    }

    #[derive(Debug)]
    struct OuterStruct {
        inner_data: InnerStruct,
        username: String,
    }

    struct User {
        // we want struct to own all of its data
        email: String,
        _name: String,
        _age: usize,
    }

    // how to use a struct
    let outer = OuterStruct {
        inner_data: InnerStruct {
            metadata: String::from("metadata"),
            _created_at: 1234,
            _active: true,
        },
        username: String::from("username"),
    };

    // how to use specific value
    println!("the username is {}, and the metadata is {}", outer.username, outer.inner_data.metadata);
    println!("{:?}", outer);

    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value. Let’s look at some examples.

    // struct update syntax
    //  when use the `mut` keyword, entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable
    let mut user1 = User { email: String::from("test"), _name: String::from("test"), _age: 9 };
    let _ = User {
        email: String::from("another@example.com"),
        ..user1
    };
    user1.email = String::from("test2");

    // Unit-Like Structs Without Any Fields
    struct AlwaysEqual;
    impl PartialEq for AlwaysEqual {
        fn eq(&self, _: &AlwaysEqual) -> bool {
            return false;
         }
    }

    #[derive(Debug)]
    struct Point { 
        x: i32, 
        y: i32, 
    }
    
    fn print_point(p: &Point) {
        println!("{}, {}", p.x, p.y);
    }

    let mut p = Point { x: 0, y: 0 };
    let x = &mut p.x;
    // print_point(&p); // cannot borrow p as mutable cuz it is also borrowed as mutable 
    *x += 1;
    println!("{}, {}", p.x, p.y);

    // but different fields can be borrowed simultaneously
    let mut p = Point { x: 1, y: 2 };
    let x = &mut p.x;
    let y = &mut p.y;
    *x += 1;
    *y += 1;

    let mut a = Point { x: 1, y: 2 };
    a.x += 1;
    // b doesn't borrow a - it takes ownership of some fields from a!
    // The key insight is that `..a` moves fields, it doesn't borrow them
    let b = Point { y: 1, ..a };
    a.x += 1;
    println!("{}", b.x);

    // display trait vs debug trait 
    // format specifier:  {} vs {:?}
    // audience:  end user vs dev
    // auto-derive: no vs #[derive[Debug]]
}

fn simple_add (x: i32, y: i32) -> i32 {
    x + y
}

