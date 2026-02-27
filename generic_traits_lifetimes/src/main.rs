// Quiz
//     1. How to define and impl trait
//     2. How to use trait as parameters
//     3. How to use trait bound syntax
//
// use generic T over function, struct and enum, all following the name of the entity. 
// and multiple generic <T, U>
// method definition declar T just after `impl`
// monomorphization to avoid runtime overhead
use generic_traits_lifetimes::basics;

fn main() {
    let num_list = vec![2, 100, 1, 2, 3];
    println!("{}", basics::largest(&num_list));

    let char_list = vec!['a', 'b', '*', 'A'];
    println!("{}", basics::largest(&char_list));

    let mut p = basics::Point{
        x: 5,
        y: 5,
    };

    println!("the x of point is {}", p.x());
    println!("set the x of point to be 10");
    let new_x = 10;
    println!("now the x of point is {}", p.set_x(new_x));

    let p = basics::Point {x: 5.0, y: 5.0};
    println!("x = {}, y = {}", p.x, p.y);
    println!("distance = {}", basics::Distance::distance(&p));

    let string1 = String::from("long string");
    {
        let string2 = String::from("xyz");
        let s1= &string1;
        let s2= &string2;
        let _result = basics::longest(s1, s2);
    }
}
