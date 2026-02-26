use std::env;
use std::process;
use io_closure_oop_pattern_unsafe::Config;
use io_closure_oop_pattern_unsafe::trait_object::build_ui;
use io_closure_oop_pattern_unsafe::raw_pointer::create_and_deref;
use io_closure_oop_pattern_unsafe::raw_pointer::count_incr;

fn main() {
    println!("Hello, world!");

    // let args = env::args().collect::<Vec<String>>();
    // //dbg!(args);

    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    // if let Err(e) = io_closure_oop_pattern_unsafe::run(config) {
    //     println!("Application error: {e}");
    //     process::exit(1);
    // };

    // a function from submodule of root crate
    //build_ui();
    create_and_deref();
    unsafe {
        count_incr();
    }

}
