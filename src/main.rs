// use core::num::*;

fn main() {
    println!("Hello, this is a example 1");

    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11, 13, 14, 15, 16, 17, 18, 19, 20];
    let sub_arr = [6, 8, 5];
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
    }else {
      println!("False");
    }
}
