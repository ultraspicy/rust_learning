/// Table of content
///     1. Smart pointer Box<T>
///     2. Reference Counter Rc<T>
///     3. RefCell<T>
///     4. The `Deref` trait
///     5. The `Drop` trait
///
fn main() {
    // b is a box that points to a 5, which allocated on the heap
    // de-allocation happens both for the box (stored on the stack) and the data it points to (stored on the heap).
    // Boxes provide only the indirection and heap allocation
    //  They also don’t have the performance overhead
    // Box<T> v.s &T
    // Box<T> provides ownership and allows moving values across scopes,
    // while &T references allow borrowing and shared access to values without taking ownership.
    let b = Box::new(5);
    println!("{}", b);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    // this is an associated type, this is an advance feature,
    // ignore why using associated type for now
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
