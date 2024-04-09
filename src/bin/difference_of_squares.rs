fn main() {
    println!("{}", square_of_sum(5) - sum_of_squares(5));
    println!("{}", square_of_sum_2(5) - sum_of_squares_2(5));
    println!("{}", square_of_sum_3(5) - sum_of_squares_3(5));
}

fn square_of_sum(n: u64) -> u64 {
    let sum = (1..=n).sum::<u64>();
    sum.pow(2)
}

fn sum_of_squares(n: u64) -> u64 {
    (1..=n).map(|x| x * x).sum::<u64>()
}

pub fn square_of_sum_2(n: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in 1..=n {
        sum += i;
    }
    sum.pow(2)
}

pub fn sum_of_squares_2(n: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in 1..=n {
        sum += i.pow(2);
    }
    sum
}

pub fn difference(n: u64) -> u64 {
    let square_of_sum = square_of_sum(n);
    let sum_of_squares = sum_of_squares(n);
    square_of_sum - sum_of_squares
}


fn square_of_sum_3(n: u64) -> u64 {
    // let sum = (1..=n).fold(0, |acc, x| acc + x);
    let sum = (1..=n).reduce(|acc, x| acc + x).unwrap();
    sum.pow(2)
}

fn sum_of_squares_3(n: u64) -> u64 {
    // (1..=n).reduce(|acc, x| acc + (x*x)).unwrap()
    (1..=n).fold(0, |acc, x| acc + (x*x))
}