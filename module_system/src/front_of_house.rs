pub mod hosting {
    pub fn add_to_waitlist() {
        println!("add_to_waitlist called");
        /// relative path starting from its parent
        super::order_candy();
    }

    fn seat_at_table() {}
}
/// define a module by starting with the mod keyword
/// inside modules, we can have other modules
/// Modules are not useful only for organizing your code. They also define Rust’s privacy boundary
/// Items in a parent module can’t use the private items inside child modules
/// but items in child modules can use the items in their ancestor modules
/// mod front_of_house {

/// Exposing Paths with the pub Keyword
pub fn order_candy() {
    println!("order_candy called");
}

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}

///  we make the struct public, but the struct’s fields will still be private.
///  pub struct Breakfast {
///         pub toast: String,
///         seasonal_fruit: String,
///     }
/// v.s.
/// we make an enum public, all of its variants are then public.
/// pub enum Appetizer {
///        Soup,
///        Salad,
///     }
