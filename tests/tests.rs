#[cfg(test)]
mod macro_tests {
    use llvm_cov_358_min_repo::fizzbuz;
    use trybuild;

    #[test]
    fn it_expands_for_simple_case() {
        assert!(fizzbuz!(
            1, 2, fizz, 4, buzz, fizz, 7, 8, fizz, buzz, 11, fizz, 13, 14, fizz_buzz
        ));
    }

    #[test]
    fn trybuild_cases() {
        let t = trybuild::TestCases::new();
        t.pass("tests/pass/*.rs");
        t.compile_fail("tests/fail/*.rs");
    }
}
