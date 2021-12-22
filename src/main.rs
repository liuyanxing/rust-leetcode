mod problems;
use problems::combination_sum::combination_sum;

fn main() {
    let candidate = vec![2, 3, 5, 7];
    let target = 7;
    let res = combination_sum(candidate, target);
    println!("{:?}", res);
}
