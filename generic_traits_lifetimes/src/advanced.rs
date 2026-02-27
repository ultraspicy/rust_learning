use std::ops::Add;


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
// trait Add<Rhs = Self> {
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
//  If Output were a generic param
// you will see shit impl like `impl Add<FloatPoint, String> for IntPoint { ... }`
impl Add<FloatPoint> for IntPoint {
    type Output = FloatPoint;

    fn add(self, rhs: FloatPoint) -> Self::Output {
        FloatPoint {
            x: self.x as f32 + rhs.x,
            y: self.y as f32+ rhs.y,
        }
    }
}
