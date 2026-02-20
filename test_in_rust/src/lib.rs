pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, another: &Rectangle) -> bool {
        self.height > another.height && self.width > another.width
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 0 {
            panic!("the value should not be less than 0, the acutal value is {}", value);
        }
        Guess{value}
    }
}

// using the cfg attribute, Cargo compiles our test code only if we actively run the tests with cargo test
#[cfg(test)]
mod tests {
    // attribute indicates this is a test function, so the test runner knows to treat this function as a test.
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    use super:: *;
    #[test]
    fn large_hold_small() {
        let l = Rectangle {
            width: 10,
            height: 10,
        };
        let s = Rectangle {
            width: 1,
            height: 1,
        };
        assert!(l.can_hold(&s));
    }

    #[test]
    #[should_panic(expected = "the value should not be less than 0")]
    #[ignore]
    fn less_than_zero() {
        Guess::new(-2);
    }
}

// Some command line options go to `cargo test`, and some go to the resulting test binary.
// To separate these two types of arguments, you list the arguments that go to cargo test
// followed by the separator --
// $ cargo test -- --test-threads=1
// $ cargo test -- --show-output
// $ cargo test <test_function_name_pattern>
// $ cargo test -- --ignored
// $ cargo test -- --include-ignored

// “Ignoring Some Tests" cargo test -- --ignored
// in the “Running a Subset of Tests by Name”
// benchmark tests
// “Running a Subset of Tests by Name”  cargo test <testname> filters by checking if the test name contains the string you provide 
// “Documentation Comments as Tests” 

// mod common; — loads/declares the module

// Tells Rust "go find this file and compile it as part of this crate"
// Without this, the file doesn't exist as far as Rust is concerned
// Only needs to happen once in the whole crate

// use common::setup; — brings into scope

// Just creates a shortcut so you can write setup() instead of common::setup()
// The module must already be loaded via mod first
// purely convenience, doesn't load anything new