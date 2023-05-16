use num_traits::Float;
/// Quiz
///     1. How to define and impl trait
///     2. How to use trait as parameters
///     3. How to use trait bound syntax
///

fn main() {
    let num_list = vec![2, 100, 1, 2, 3];
    println!("{}", largest(&num_list));

    let char_list = vec!['a', 'b', '*', 'A'];
    println!("{}", largest(&char_list));

    let mut p = Point{
        x: 5,
        y: 5,
    };

    println!("the x of point is {}", p.x());
    println!("set the x of point to be 10");
    let new_x = 10;
    println!("now the x of point is {}", p.set_x(new_x));

    let p = Point {x: 5.0, y: 5.0};
    println!("x = {}, y = {}", p.x, p.y);
    println!("distance = {}", p.distance());
}

fn largest<T: std::cmp::PartialOrd>(list :&[T]) -> &T {
    let mut ret = &list[0];
    for num in list {
        if num > ret {
            ret = num
        }
    }
    return ret
}

struct Point<T> {
    x: T,
    y: T,
}

pub trait Distance<T: Float> {
    fn distance(&self) -> T;
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn set_x(&mut self, new_x: T) -> &T {
        self.x = new_x;
        &self.x
    }
}

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
// fn longer<'a> (s1: &'a str, s2: &'a str) -> &'a str {
//     if s1.len() > s2.len() {
//         s1
//     } else {
//         s2
//     }
// }
// lifetime in struct
// it means an instance of Something can’t outlive the reference it holds in its part field.
// struct Something<'a> {
//     part: &'a str,
// }
// // interesting lifetime elision rule, which makes the method much nicer to review and write
//
// // the lifetime of all string literals is 'static., living for the entire duration of the program
//
// // put generics, trait bounds and lifetime together
// fn longer_refined<'a, T> (
//     s1: &'a str,
//     s2: &'a str,
//     announce: T
// ) -> &'a str where T:Display {
//     println!("announce {}", announce);
//     if s1.len() > s2.len() {
//         s1
//     } else {
//         s2
//     }
// }