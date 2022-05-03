use num;

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

/// trait tells the Rust compiler about functionality a particular type has
/// define & implementing
pub trait Distance {
    fn distance(&self) -> f32 {
        // this is a default implementation
        0.0
    }
}

impl<T: num::Integer + std::ops::Mul<Output = &T>, U: num::Float> Distance for Point<T, U> {
    fn distance(&self) -> f32 {
        &self.x * &self.x + &self.y * &self.y
    }
}
// fn largest_generic<T>(list: &[T]) -> T {
//     let mut ret = list[0];
//
//     for &item in list {
//         if item > ret {
//             ret = item;
//         }
//     }
//     ret
// }