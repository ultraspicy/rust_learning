
/// closure is a succinct type of function captures a narrow context
/// it doesn't have to be type-annotated since it won't be exposed to users
/// code sample
///   let example_closure = |x| x + 1;
/// closure can capture variable from env in three ways just as functions do
///    aka borrowing immutably, borrowing mutably, and taking ownership
///    and closure will decide which of these to use base on the function body
/// use keyword `move` to forcely move the ownership like the following
///     move || println!("From thread {:?}", list)
///
/// closure has traits
///     - FnOnce - for closures that can be called once, for closures that moves captured values
///     - FnMut - for closures mutate the captured values. Those closures can be called more than once
///     - Fn - for closures don't move or mutate captured values, or capture nothing from env.
///
/// todo lifetime and lifetime, when closure taking a string ref, the ref need to live longer than the closure
///
///
///
fn main() {
    let v1 = vec![1, 2, 3];
    //  make v1_iter mutable: calling the next method on an iterator changes internal state that the iterator uses
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));

    // The iter() method produces an iterator over immutable references. 
    // If we want to create an iterator that takes ownership of v1 and returns owned values, we can call into_iter()
    // Similarly, if we want to iterate over mutable references, we can call iter_mut() instead of iter.

    let total: i32 = v1_iter.sum(); // sum takes ownership of the iterator
    println!("Total: {}", total); // Total: 3 cause next() consumes 

    // calling the iterator adaptor method map
    let v2 = vec![4, 5, 6];
    let mut v2_iter = v2.into_iter().map(|x|x *2);
    println!("v2 {:?}", v2_iter.next());
}
