mod front_of_house;

/// idiomatic way to bring a function into scope with `use`
/// then `hosting::add_to_waitlist()`
/// when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path.
/// like `use std::collections::HashMap;`
/// Providing New Names with the as Keyword
/// combine pub and use. This technique is called re-exporting
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}