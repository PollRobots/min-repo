use llvm_cov_358_min_repo::fizzbuz;

pub fn main() {
    let _ = fizzbuz! {
        1, 2, FIZZ, 4, BUZZ, Fizz, 7, 8, not, valid, 11, fizz, 13, 14, FizzBuzz
    };
}
