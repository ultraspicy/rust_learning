
use num_traits::Float;
use std::fmt::Display;

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

pub fn largest<T: std::cmp::PartialOrd>(list :&[T]) -> &T {
    let mut ret = &list[0];
    for num in list {
        if num > ret {
            ret = num
        }
    }
    return ret
}

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub trait Distance<T: Float> {
    fn distance(&self) -> T;
}

impl<T> Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn set_x(&mut self, new_x: T) -> &T {
        self.x = new_x;
        &self.x
    }
}

// The visibility is determined by the trait itself, not the implementation.
impl<T: Float> Distance<T> for Point<T> {
    fn distance(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}


// &i32  a reference
// &'a i32  a reference with an explicit lifetime annotation
// &'a mut i32  a mutable reference with an explicit lifetime annotation

// lifetime in function signature
// 1. we’re not changing the lifetimes of any values passed in or returned.
// Rather, we’re specifying that the borrow checker should reject
// any values that don’t adhere to these constraints
// 2 .returning a reference from a function, the lifetime parameter for the return type
// needs to match the lifetime parameter for one of the parameters.
fn _longer<'a> (s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
// lifetime in struct
// it means an instance of Something can’t outlive the reference it holds in its `part` field.
struct _Something<'a> {
    part: &'a str,
}
// // interesting lifetime elision rule, which makes the method much nicer to review and write
// when a method has &self or &mut self as a parameter, Rust automatically assigns the self lifetime
//  to all output references. 

// Why not just refer from the impl?
// Rust wants the contract to be explicit and stable at the boundary rather than implicitly derived from the implementation.

// // the lifetime of all string literals is 'static., living for the entire duration of the program
// String literals aren't owned by anyone — they're just pointers into the program's read-only memory.

// // put generics, trait bounds and lifetime together
fn _longer_refined<'a, T> (
    s1: &'a str,
    s2: &'a str,
    announce: T
) -> &'a str where T:Display {
    println!("announce {}", announce);
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
// Adding lifetime annotations doesn't solve a lifetime problem — it just makes the relationships explicit
// enough that Rust can detect the problem elsewhere. 
// You could think of it like type annotations: writing x: i32 doesn't fix a bug, 
// it just lets the compiler check for type mismatches. Lifetime annotations do the same thing for reference validity.
// The frustrating part is that adding annotations can reveal a new error that was always there but invisible. 
// That feels like the annotations broke something, but really they just made a latent bug visible — which is the whole point.