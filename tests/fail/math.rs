use llvm_cov_358_min_repo::fizzbuz;

pub fn main() {
    let _ = fizzbuz! {
        1,
        1,
        3,
        4,
        5,
        6,
        7,
        8,
        7,
        8,
        11,
        buzz,
        fizz,
        fizz_buzz,
        15
    };
}
