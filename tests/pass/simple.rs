use llvm_cov_358_min_repo::fizzbuz;

pub fn main() {
    let _ = fizzbuz! {
        1, 2, fizz, 4, buzz, fizz, 7, 8, fizz, buzz, 11, fizz, 13, 14, fizz_buzz
    };
}
