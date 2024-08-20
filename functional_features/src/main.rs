
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
    println!("Hello, world!");
}
