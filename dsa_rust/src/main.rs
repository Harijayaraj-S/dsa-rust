// MAIN

mod arrays;
mod basic;
mod concepts;
mod condition_loop;

fn main() {
    println!("{}", basic::odd_or_even(10));
    println!("{}", basic::greeting("John"));
    println!("{}", basic::simple_interest(10, 3, 1));
    println!("{}", basic::calculator(10, 3, "+".to_string()));
    println!("{}", basic::largest_number(10, 3));
    println!("{}", basic::currency_converter(10.0));
    println!("{}", basic::palindrome("madam".to_string()));
    println!("{}", basic::armstrong_number(153));
    println!("{}", condition_loop::area_of_circle(13.0));
    println!("{}", condition_loop::area_of_triangle(13.0, 9.0));
    println!("{}", condition_loop::area_of_rectangle(13.0, 9.0));
    println!("{}", condition_loop::area_of_isosceles_triangle(13.0, 9.0));
    println!("{}", condition_loop::area_of_parallelogram(13.0, 9.0));
    println!("{}", condition_loop::area_of_rhombus(13.0, 9.0));
    println!("{}", condition_loop::perimeter_circle(19.0));
    println!("{}", condition_loop::area_equilateral_triangle(19.0));
    println!("{}", condition_loop::perimeter_equilateral_triangle(19.0));
    println!("{}", condition_loop::perimeter_equilateral_triangle(19.0));
    println!("{}", condition_loop::perimeter_of_parallelogram(19.0, 30.0));
}
