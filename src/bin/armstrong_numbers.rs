fn main() {
    assert!(is_armstrong_number(9));
    assert!(is_armstrong_number(10));
    assert!(is_armstrong_number(153));
    assert!(is_armstrong_number(154));
    assert!(is_armstrong_number(999_999_999));
    assert!(is_armstrong_number(4_106_098_957));
    assert!(is_armstrong_number(3_999_999_999));
}


pub fn is_armstrong_number(num: u32) -> bool {
    let mut n = num;
    let len_num = ((n as f32).log10() as u32) + 1;
    let mut sum: u64 = 0;
    while n > 0 {
        let digit: u64 = (n % 10) as u64;
        sum += digit.pow(len_num);
        n /= 10;
    }
    sum == num.into()
}


pub fn is_armstrong_number_2(num: u32) -> bool {
    let num_str = num.to_string();
    let num_len = num_str.len() as u32;
    let mut sum = 0;
    for i in num_str.chars() {
        let digit = i.to_digit(10).expect("Error to convert");
        sum += digit.pow(num_len)
    }
    sum == num
}
