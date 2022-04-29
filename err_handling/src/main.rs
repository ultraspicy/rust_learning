use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    /// Result value is the Ok variant, unwrap will return the value inside the Ok.
    /// If the Result is the Err variant, unwrap will call the panic! macro for us.
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    /// A Shortcut for Propagating Errors: the ? Operator
    /// ? placed after a Result value
    /// If the value of the Result is an Ok, the value inside the Ok will get returned from this expression
    /// If the value is an Err, the Err will be returned from the whole function
    /// The ? operator can only be used in functions whose return type is compatible with the value the ? is used on
    ///     let mut s = String::new();
    ///     File::open("hello.txt")?.read_to_string(&mut s)?;
    ///     Ok(s)
}
