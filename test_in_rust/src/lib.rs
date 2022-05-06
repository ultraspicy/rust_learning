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