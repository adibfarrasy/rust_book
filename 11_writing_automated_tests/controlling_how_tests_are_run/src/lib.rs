fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

// use --test-threads=1 to run tests synchronously
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        // --show-output will show 4
        let value = prints_and_returns_10(4);
        assert_eq!(10, value)
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value)
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    } // run using --include-ignored or --ignored
}
