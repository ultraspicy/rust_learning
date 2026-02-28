use core::fmt;
use std::ops::Add;

// topic 1: associate type and operator overloading
// use associated type to bind a trait with a specfic type
// Implementation can only associate Iterator trait with one type once
// since this trait doesn't have a generic param in its definition
// rust treats Iter<String> and Iter<i32> two different types
// but two Item type assignment is impossible
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct IntPoint {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct FloatPoint {
    x: f32,
    y: f32,
}


// operator overloading
 // trait definition — Rhs is declared here as a generic param
// trait Add<Rhs = Self> { // default type parameters.
//     fn add(self, rhs: Rhs) -> ...
// }
impl Add for IntPoint {
    type Output = IntPoint;

    fn add(self, rhs: Self) -> Self::Output {
        IntPoint {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// again, one trait one associate type
// Associated type enforces that the relationship (Self, Rhs) → Output
// is unique and decided by the implementor, not the caller
// If Output were a generic param
// you will see shit impl like `impl Add<FloatPoint, String> for IntPoint { ... }`

// core benefit of using associate types over generics
// the same operation IntPoint + FloatPoint has two completely different behaviors depending on what output you want. That's confusing and error-prone.
// Associated types forbid this at the impl level — you simply cannot write two impls with different Output for the same (Self, Rhs) pair. The compiler rejects it outright.
impl Add<FloatPoint> for IntPoint {
    type Output = FloatPoint;

    fn add(self, rhs: FloatPoint) -> Self::Output {
        FloatPoint {
            x: self.x as f32 + rhs.x,
            y: self.y as f32+ rhs.y,
        }
    }
}

// topic 2: disambiguity
// disambiguity dup names
// for methd
// Dog::bark(&dog);
// for non method, use fully qualified syntax
// <Type as Trait>::function(receiver_if_method, next_arg, ...);
// println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
pub struct Dog {
    name: String
}

trait Animal {
    fn make_noise();
    fn call_my_name(&self);
}

impl Dog {
    fn make_noise() {
        println!("woof woof");
    }

    fn call_my_name(&self) {
        println!("i am puppy with name {}", self.name);
    }
}

impl Animal for Dog {
    fn make_noise() {
        println!("wooooooooooooo, I am an wild animal");
    }

    fn call_my_name(&self) {
        println!("stop, I am an wild animal");
    }
}

pub fn how_to_dedup_fn_name() {
    let dog = Dog{name: String::from("mika")};

    // for method
    Animal::call_my_name(&dog);
    Dog::call_my_name(&dog);

    // for non method, use fully qualified syntax
    <Dog as Animal>::make_noise();
    Dog::make_noise();
}

//topic 3: Using Supertraits
trait PrintWithFrame: fmt::Display {
    fn print(&self) {
        println!("*****************");
        println!("**     {}      **", self.to_string());
        println!("*****************");
    }
}

impl fmt::Display for IntPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, ...) writes into the formatter f
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PrintWithFrame for IntPoint {}

pub fn run_frame_print() {
    let p: IntPoint = IntPoint{x: 1, y: 2};
    p.print();
}
