fn main(){
    println!("{}", reverse("input"));
    println!("{}", reverse_2("input"));
    println!("{}", reverse_3("input"));
}

pub fn reverse(input: &str) -> String {
    let mut reversed = String::new();
    // chars returns an Iterator, and rev works on it from right to left
    for c in input.chars().rev(){
        reversed.push(c);
    }
    reversed
}

pub fn reverse_2(input: &str) -> String {
    // collect transforms an iterator into a collection
    input.chars().rev().collect()
}

pub fn reverse_3(input: &str) -> String {
    input.bytes().rev().map(|b| b as char).collect()
}