use std::env;
use std::fs;
use std::io;
use std::fmt::Display;

fn main() {
    println!("Hello, this is a example 1");

    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11, 13, 14, 15, 16, 17, 18, 19, 20];
    let sub_arr = [8,10,11];
    let num_loops = org_arr.len() - 2;
    let mut bool = false;

    for n in 0..num_loops {
        let i = n;

        let j = i + 3;
        let slice_arr = &org_arr[i..j];

        if sub_arr == slice_arr {
            bool = true;
        }
    }
    if bool == true {
        println!("True");
    } else {
        println!("False");
    }

    println!("Hello, this is a example 2");

    // let contents = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
    // let str_chars = contents.chars();

    // let mut line = String::new();
    // println!("Enter the term you want to find: ");
    // io::stdin().read_line(&mut line).unwrap();
    // line.pop();

    // let mut number_matches = 0;

    let contents = fs::read_to_string("bt2.txt").expect("Something went wrong reading the file");

    println!("Please input your term: ");
    let mut input_term = String::new();
    io::stdin().read_line(&mut input_term).unwrap();
    input_term.pop();
    println!("Your term is: {}", input_term);

    let num_matches = contents.matches(&input_term.trim()).count();

    println!("Number of matches: {}", num_matches);
}
