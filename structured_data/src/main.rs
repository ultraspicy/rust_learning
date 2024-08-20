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
        created_at: u64,
        active: bool,
    }

    #[derive(Debug)]
    struct OuterStruct {
        inner_data: InnerStruct,
        username: String,
    }

    // how to use a struct
    let outer = OuterStruct {
        inner_data: InnerStruct {
            metadata: String::from("metadata"),
            created_at: 1234,
            active: true,
        },
        username: String::from("username"),
    };

    // how to use specific value
    println!("the username is {}, and the metadata is {}", outer.username, outer.inner_data.metadata);
    println!("{:?}", outer);

    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value. Let’s look at some examples.

    // struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

fn simple_add (x: i32, y: i32) -> i32 {
    x + y
}
