use std::fmt::Display;
use num_traits;

fn main() {
    let num_list = vec![1, 2, 3];
    println!("{}", largest(&num_list));

    let p = Point {x: 5, y: 3.5};
    println!("x = {}, y = {}", p.x, p.y);
    println!("distance = {}", p.distance());
}

fn largest(list: &[i32]) -> i32 {
    let mut ret = list[0];

    // We are not referencing a reference to an i32
    // pattern matching and destructuring each &i32 that the for loop gets so that item will be an i32
    for &num in list {
        if ret < num {
            ret = num;
        }
    }
    ret
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

// trait tells the Rust compiler about functionality a particular type has
// defining & implementing a trait
// we can implement a trait on a type only if at least one of the trait or the type is local to our crate
pub trait Distance {
    fn distance(&self) -> f32 {
        // this is a default implementation
        0.0
    }
}

impl<'a, T: num_traits::FromPrimitive + std::ops::Mul<Output = &'a T>,
    U: num_traits::FromPrimitive
> Distance for Point<T, U> {
    fn distance(&self) -> f32 {
        &self.x * &self.x + &self.y * &self.y
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
fn longer<'a> (s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
// lifetime in struct
// it means an instance of Something can’t outlive the reference it holds in its part field.
struct Something<'a> {
    part: &'a str,
}
// interesting lifetime elision rule, which makes the method much nicer to review and write

// the lifetime of all string literals is 'static., living for the entire duration of the program

// put generics, trait bounds and lifetime together
fn longer_refined<'a, T> (
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