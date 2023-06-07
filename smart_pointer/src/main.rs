fn main() {
    // b is a box that points to a 5, which allocated on the heap
    // deallocation happens both for the box (stored on the stack) and the data it points to (stored on the heap).
    // Boxes provide only the indirection and heap allocation
    //  They also don’t have the performance overhead

    // Box<T> provides ownership and allows moving values across scopes,
    // while &T references allow borrowing and shared access to values without taking ownership.
    let b = Box::new(5);
    println!("{}", b);

}
