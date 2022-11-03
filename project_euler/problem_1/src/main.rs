//https://projecteuler.net/problem=1

const target: i64 = 999;

fn sum_divisible_by(n: i64) -> i64 {
    let l = target / n;
    n * l * (l + 1) / 2
}

fn main() {
    println!(
        "{}",
        sum_divisible_by(3) + sum_divisible_by(5) - sum_divisible_by(15)
    )
}

#[test]
fn test_sum_divisible_by() {
    assert_eq!(
        sum_divisible_by(3) + sum_divisible_by(5) - sum_divisible_by(15),
        233168
    );
}
