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

    // Result value is the Ok variant, unwrap will return the value inside the Ok.
    // If the Result is the Err variant, unwrap will call the panic! macro for us.
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // A Shortcut for Propagating Errors: the ? Operator
    // ? placed after a Result value
    // If the value of the Result is an Ok, the value inside the Ok will get returned from this expression
    // If the value is an Err, the Err will be returned from the whole function
    // The ? operator can only be used in functions whose return type is compatible with the value the ? is used on
    //     let mut s = String::new();
    //     File::open("hello.txt")?.read_to_string(&mut s)?;
    //     Ok(s)

//     The From conversion part is what makes ? flexible. As long as the error type of what you're ?-ing can be converted into the error type of your function's return type, it works. For example:
// rustfn foo() -> Result<(), MyError> {
//     let f = std::fs::File::open("file.txt")?;  // io::Error → MyError via From
//     Ok(())
// }
// As long as you implement From<io::Error> for MyError, the ? handles the conversion automatically.

    // unwrap(): panic on None/Error
    // expect(msg): panic on None/Error with msg
    // unwrap_or(val: T): return val on on None/Error 
    // unwrap_or_else(f) call f/f(err) on None/Error 
    // unwrap_or_default() return T::default() on None/Error 

    // ok_or(e: E) Option<T> -> Result<T, E>
    // Converts None to Err(err)
    // is_some(), is_none()
    let opt = Some(5);
    let rst = opt.ok_or("option is none");
    println!("rst = {:?}, is_ok() = {}", rst, rst.is_ok());

    let opt2: Option<i32> = None;
    let rst = opt2.ok_or("option is none");
    println!("rst = {:?}, is_err() = {}", rst, rst.is_err());

    // ok() Result<T, E> -> Option<T>
    // Converts Err to None, discards error
    // is_ok(), is_err()
    let rst: Result<i32, String> = Ok(5);
    let opt = rst.ok();
    println!("opt = {:?}", opt);

    let rst: Result<i32, String> = Err(String::from("oh no"));
    let opt = rst.ok();
    println!("opt = {:?}", opt);

    // as_ref(): &Option<T> -> Option<&T> or &Result<T, E> -> Result<&T, &E>
    // work with references to the inner values without moving/consuming the original.
    let opt_ref: &Option<String> = &Some(String::from("this is a dog"));
    let as_ref_opt = opt_ref.as_ref().map(|s| s.chars().last().unwrap());
    println!("as_ref_opt = {:?}", as_ref_opt);

    // as_mut() Converts from &mut Option<T> to Option<&mut T> (or &mut Result<T, E> to Result<&mut T, &mut E>).
    let opt_ref_mut: &mut Option<String> = &mut Some(String::from("this is a cat"));
    let as_mut_opt = opt_ref_mut.as_mut().map(|s| s.chars().map(|c| (c as u8 + 1) as char).collect::<String>());
    println!("as_mut_opt = {:?}", as_mut_opt);
}
