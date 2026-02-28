// declarative macros with macro_rules! and three kinds of procedural macros:
// Custom #[derive] macros for General Metaprogramming
// Procedure macros for Generating Code from Attributes
// Macros run at compile time, before the compiler interprets the code.

// Rust's macro definition syntax 

// #[macro_export]
// Makes the macro available to other crates that import your crate. 
// Without it, the macro is only usable within the crate it's defined in.
#[macro_export] // 
macro_rules! vec {
    // $( ... ),* — repeat the pattern inside zero or more times, separated by ,
    ( $( $x:expr ),* ) => { // :expr is a fragment specifier telling the macro "match anything that is a valid Rust expression
        {
            let mut temp_vec = Vec::new();
            // *expand* the captured variables. `(tempv​ec.push($x);)*` means "repeat this statement for each
            $( 
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// procedural macros can be categorized to custom derive, attribute-like, and function-like