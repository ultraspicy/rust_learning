fn main() {

    /// Heap memory must be requested from the memory allocator at runtime.
    let mut s = String::from("hello");
    s.push_str(", world");

    println!("{}", s);

    ///     let s1 = String::from("hello");
    ///     let s2 = s1;
    ///
    ///     println!("{}, world!", s1); -> error value borrowed here after move
    /// s1 moves to s2
    /// but
    /// If a type implements the Copy trait, a variable is still valid after assignment to another variable.
    /// Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait, such as String
    /// A type that implements Copy must also implement Clone
    /// because a type that implements Copy has a trivial implementation of Clone that performs the same task as Copy.

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    /// Passing a variable to a function will move or copy, just as assignment does.
    ///     let s = String::from("hello");   s comes into scope
    ///     takes_ownership(s);              s's value moves into the function and so is no longer valid
    /// Returning values can also transfer ownership
    /// use reference to avoid move (making variable invalid after assigning)
    ///  reference and borrowing - We call the action of creating a reference borrowing
    /// which allow using a value without transferring ownership

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    /// mutable reference
    /// reference’s scope starts from where it is introduced and continues through the last time that reference is used
    /// we can have only one mutable reference to a particular piece of data at a time.
    ///    let r1 = &mut s;
    ///    let r2 = &mut s;
    ///    println!("{}, {}", r1, r2); -> two mutable reference
    /// we cannot have a mutable reference while we have an immutable one to the same value
    let mut s = String::from("hello");
    change_something(&mut s);
    println!("{}", s);

    let s = String::from("hello");

    /// let mut s = String::from("hello world"); s is an immutable reference
    /// let word = first_word(&s);
    /// s.clear() uses a mutable reference
    /// println!("the first word is: {}", word); error occurs, no mutable reference and immutable one at the same time

    let my_string = String::from("hello world");
    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent to whole slices of `String`s
    let word = first_word(&my_string);
    let my_string_literal = "hello world";
    // Because string literals *are* String slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

// Defining a function to take a string slice instead of a reference to a String makes our API more general
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
}

fn change_something(some_string: &mut String) {
    some_string.push_str(", world");
}
