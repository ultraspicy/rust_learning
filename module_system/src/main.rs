use module_system::eat_at_restaurant;

fn main() {
    println!("this is a test");
    eat_at_restaurant();
}

// Question 1: both src/main.rs and src/lib.rs
// You get two crates in the same package — a binary crate and a library crate, both with the same name as the package. The binary (main.rs) can use the library (lib.rs) as an external dependency:
// rust// in main.rs
// use my_package::some_function;  // using your own lib crate
// This is actually a very common pattern — put all your logic in lib.rs so it's testable and reusable, and keep main.rs thin as just the entry point.

// Question 2: what is "the name of the package"
// It's whatever you named it in cargo new. So cargo new my_app gives you a package named my_app, and your crates are also named my_app. You can see it in Cargo.toml:
// toml[package]
// name = "my_app"   # ← this is the package name
// So src/main.rs compiles to a binary called my_app, and src/lib.rs becomes a library crate you import as use my_app::....

// Question 3: both src/garden.rs and src/garden/mod.rs
// You get a compiler error. Rust does not allow both to exist at the same time — they're two ways of doing the same thing and you must pick one. The modern convention (Rust 2018+) is to prefer src/garden.rs. The mod.rs style is older and still works but is considered outdated.

// Question 4: where to put pub use
// Typically in your crate root (src/lib.rs), because that's what external users of your library see. The idea is your internal structure can be organized however you want, but you re-export a clean public API from the top level:
// rust// src/lib.rs
// mod garden;           // internal structure
// mod utils;

// pub use garden::vegetables::grow;   // clean public API
// pub use garden::vegetables::Water;
// Now users just write use my_crate::grow instead of use my_crate::garden::vegetables::grow. They don't need to know your internal folder structure.

// Question 5: [feature = "abc"]
// That's a conditional compilation attribute, using Cargo's feature flags system. You define optional features in Cargo.toml:
// toml[features]
// abc = []                        # feature with no dependencies
// json_support = ["serde_json"]   # feature that pulls in a dependency
// Then in code you gate things behind it:
// rust#[cfg(feature = "abc")]
// pub fn only_when_abc_enabled() { }

// #[cfg(feature = "json_support")]
// use serde_json::Value;
// Users of your crate opt into features explicitly:
// toml# in their Cargo.toml
// [dependencies]
// my_crate = { version = "1.0", features = ["abc", "json_support"] }
// The common use case is keeping optional/heavy dependencies out of the default build. For example, serde (serialization library) uses features heavily so you only pull in what you actually need.