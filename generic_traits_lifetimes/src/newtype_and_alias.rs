// new type can expose a public API that is different from the API of the private inner type.
// Newtypes can also hide internal implementation

use std::clone;

// use struct Wrapper(T) for newtypes, and use type A = B for alias
// newtype
struct _I32(i32);

// type alias for synonyms
type _Result<T> = std::result::Result<T, std::io::Error>;

// The Never Type That Never Returns
fn bar() -> ! {
    panic!("it never returns")
}

// Dynamically Sized Types and the Sized Trait
fn generic<T: Sized>(t: T) {
    // --snip--
}

fn generic_unsized<T: ?Sized>(t: &T) {
    // --snip--
}

// FnOnce — consumes captured variables, can only be called once
// FnMut — mutably borrows captured variables
// Fn — immutably borrows captured variables (most restrictive, most reusable)
// Function pointers (fn) don't capture anything, so they trivially implement all three.

// Accept only function pointers (closures may not work):
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 { 0 }
// Accept any closure or function pointer (preferred, works for both):
fn do_twice_closure_form<F: Fn(i32) -> i32>(f: F, arg: i32) -> i32 { 0 }

// Returnning closure will be a opaque but distinct type, even if their sig are same
fn returning_closure () -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn returning_closure_different_opaque_type() -> impl Fn(i32) -> i32 {
    |x| x + 2
}

fn organize_closures () {
    // code wont work since two closures are different types
    // let closures = vec![returning_closure(), returning_closure_different_opaque_type()];

    // so we need to use trait object
    let closures: Vec<Box<dyn Fn(i32)-> i32>> = vec![Box::new(returning_closure()), Box::new(returning_closure_different_opaque_type())];
}
