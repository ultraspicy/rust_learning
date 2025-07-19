fn main() {

    // Quiz
    //     1. What is move
    //     2. What is the difference between value copy vs. clone. Why Copy trait and Drop trait are mutually exclusive
    //     3. What is the implication of function arguments passing and returning
    //     4. How to use value without transferring ownership
    //     5. What's the limitation of mutable reference
    //     6. What's the scope of a reference
    //     7. Heap v.s. stack, what is allocating on the heap

    // Heap memory must be requested from the memory allocator at runtime.
    let mut s = String::from("hello");
    s.push_str(", world");

    println!("{}", s);

    // What is a move? move the ownership of the heap data \n
    // Clone trait: get a 'deep copy' of the heap data as well.
    // Copy trait: if a type implements the Copy trait, variables that use it do not move but rather are trivially copied, making them still valid after assignment to another variable.
    // Drop trait: if a type implements the Drop trait, variables will be recycled when go out of scope

    //     let s1 = String::from("hello");
    //     let s2 = s1;
    //
    //     println!("{}, world!", s1); -> error value borrowed here after move
    // s1 moves to s2
    // but
    // If a type implements the Copy trait, a variable is still valid after assignment to another variable.
    // Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait, such as String
    // A type that implements Copy must also implement Clone
    // because a type that implements Copy has a trivial implementation of Clone that performs the same task as Copy.

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // Passing a variable to a function will move or copy, just as assignment does.
    //     let s = String::from("hello");   s comes into scope
    //     takes_ownership(s);              s's value moves into the function and so is no longer valid
    // Returning values can also transfer ownership
    // use reference to avoid move
    //  reference(non-owning pointers) and borrowing - We call the action of creating a reference borrowing
    // which allow using a value without transferring ownership
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // mutable reference
    // reference’s scope starts from where it is introduced and continues through the last time that reference is used
    // we can have only one mutable reference to a particular piece of data at a time.
    //    let r1 = &mut s;
    //    let r2 = &mut s;
    //    println!("{}, {}", r1, r2); -> two mutable reference
    // we cannot have a mutable reference while we have an immutable one to the same value, cuz it may lead to deref a deallocated memory
    let mut s = String::from("hello");
    change_something(&mut s);
    println!("{}", s);

    let s = String::from("hello");

    // let mut s = String::from("hello world");
    // let word = first_word(&s); immutable reference
    // s.clear() uses a mutable reference
    // println!("the first word is: {}", word); error occurs, no mutable reference and immutable one at the same time
    // Q: what is String and what is &str
    // A: String is a type that will always be allocated on heap
    // while &str stands for string slice, a "fat" pointer with a `len` field

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

    println!(" ------ test the reference lifetime------");
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("the first word is: {}", word);
    // s.clear(); this will cause error
    println!("the first word is: {}", word);
    println!("{x}")
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

// additional notes:
// Box deallocation principle: If a variable owns a box, when Rust deallocates the variable’s frame, 
// then Rust deallocates the box’s heap memory.
// Clone() does not always make an independent replica on heap. For example 
// let data = Rc::new(vec![1, 2, 3]);
// let data2 = data.clone();
// this will end up with 2 stack variables

// reference will change the permission
// immutable reference will have read permission, but at the same time invalidation Write (if var is mut) and Own from owner
// mutable reference will have RW. Also, you cannot create a mutbale reference on immutable variable
// structs, enums, and traits, those features will have specific interactions with ownership