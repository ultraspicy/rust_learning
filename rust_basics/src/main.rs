use std::net::IpAddr;

fn main() {

/// Quiz:
///  1. given variable are immutable by default, what is the difference between a variable and a constant
///  2. what is shadowing
///  3. how to do type annotation
///  4. what is the difference between a statement and an expression (Expressions do not include ending semicolons!!!)

/// Rust is a statically typed language
/// Scalar Types - integers, floating-point, booleans, characters
///     Integers - Decimal 98_222, Hex 0xff, Binary 0b1111_0000, Byte b'A'
///     Booleans are one byte in size
///     Rust’s char type is four bytes in size and represents a Unicode Scalar Value, a lot more than just ASCII

    /// A tuple is a general way of grouping together a number of values with a variety of types into one compound type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let five_hundred = tup.0;

    /// array in Rust have a fixed length.
    let a = [1, 2, 3, 4, 5];
    let first = a[0];

    /// Statements are instructions that perform some action and do not return a value.
    /// so you can’t assign a let statement
    /// Expressions evaluate to a resulting value.


/// struct and enums
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    /// tuple struct
    struct Color(i32, i32, i32);
    /// Unit-Like Structs
    struct AlwaysEqual;

    /// Everything within this impl block will be associated with the Rectangle type.
    impl User {
        /// method
        fn get_email(&self) -> &str {
            self.email.as_str()
        }
        /// associated function
        fn admin (email: String, username: String) -> User {
            User {
                email,
                username,
                active: true,
                sign_in_count: 0,
            }
        }
    }

    enum IpAddress {
        /// put any kind of data inside an enum variant
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddress::V4(127, 0, 0, 1);
    let loopback = IpAddress::V6(String::from("::1"));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    /// catch-all pattern
    let dice_roll = 9;
    match dice_roll {
        3 => println!("hit {}", dice_roll),
        7 => println!("hit hit {}", dice_roll),
        other => println!("go to the other arm, {:?},", other),
    }

    println!("{TWO}");
}
const TWO: u32 = 1 + 1;
