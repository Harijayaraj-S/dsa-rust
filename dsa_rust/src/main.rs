// MAIN

mod basic;
fn main() {
    println!("{}", basic::odd_or_even(10));
    println!("{}", basic::greeting("John"));
    println!("{}", basic::simple_interest(10, 3, 1));
    println!("{}", basic::calculator(10, 3, "+".to_string()));
    println!("{}", basic::largest_number(10, 3));
    println!("{}", basic::currency_converter(10.0));
    println!("{}", basic::palindrome("madam".to_string()));
    println!("{}", basic::armstrong_number(153));
}
