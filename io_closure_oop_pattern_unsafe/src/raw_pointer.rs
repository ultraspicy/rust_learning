// static = Data that lives for the entire program lifetime
// Single memory location, lives forever
static mut COUNTER_COUNTER_CHICKEN_FINGER: usize = 0;

// fn — "You can call me from anywhere, I guarantee safety to the outside world"
// unsafe fn — "You must uphold certain invariants before calling me"
// create_and_deref is safe because the caller doesn't need to worry about anything.
// All the unsafe stuff is handled internally — the author has audited it and is saying
// "I've wrapped the danger, you don't need to care."
// Rust's safety model — safe abstractions over unsafe internals:
pub fn create_and_deref() {
    let mut num = -5;
    let _r1 = &raw const num;
    let _r2 = &raw mut num;

    unsafe {
        // raw pointer can only deref'd in unsafe block
        println!("r1 = {}", *_r1);
        println!("r2 = {}", *_r2);

        // unsafe function can only be called in unsafe block
        dangerous();
    }

    let mut vec = vec![1,2,3,4,5,6];
    let (a, b) = vec.split_at_mut(3);
    println!("a = {:?}", a);
    println!("b = {:?}", b);

    unsafe {
        let abs = abs(num);
        println!("abs = {}", abs);

        //totally_fake_function();
        let (a, b) = (-1, -2);
        println!("custom_abs = {}", custom_abs(a));
        println!("custom_add = {}", custom_add(a, b));
    }


}

unsafe fn dangerous(){}

unsafe extern "C" {
    // At the linker level, C symbols are just names 'abc'
    // compiler will trust the author for the existence/validity of function
    fn abs(input: i32) -> usize;

    // if we declare a fake function, compiler will compile
    // but link process will error out
    // fn totally_fake_function() -> i32;

    // this is how we define customized function
    // compile just trusts you, if you claim it as usize (whereas add.c use int)
    // compiler will still run the executable, and cast that memory cell into a usize even if it
    // is not the data type we declare in C
    fn custom_abs(input: i32) -> usize;
    fn custom_add(a:i32, b: i32) -> usize; // wrong
}

// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time.
pub unsafe fn count_incr() {
    unsafe {
        COUNTER_COUNTER_CHICKEN_FINGER += 1;
        // here a raw pointer is requried to create reference of mutable static variable
        println!("now the COUNTER_COUNTER_CHICKEN_FINGER = {}", *(&raw const COUNTER_COUNTER_CHICKEN_FINGER) );
    }

}
