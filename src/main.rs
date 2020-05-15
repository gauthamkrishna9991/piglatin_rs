/*
 *   Copyright (c) 2020 Goutham Krishna K V <gauthamkrishna9991@live.com>
 *   All rights reserved.
 */

use std::io::{self as stdio, Write};
// Write imported for flush

fn main() {
    let mut s = String::new();
    print!("Give me a word for finding pig latin : ");
    stdio::stdout().flush().unwrap();
    stdio::stdin()
        .read_line(&mut s)
        .expect("Error in getting line");
    match s.trim().to_lowercase().chars().nth(0) {
        None => {
            println!("Unknown error. Program Exiting");
        }
        Some(x) => {
            let mut newstr = String::new();
            match x {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    newstr.push_str(&s.trim()[..]);
                    newstr.push_str("-hay");
                }
                _ => {
                    newstr.push_str(&s.trim()[1..]);
                    newstr.push('-');
                    newstr.push(x);
                    newstr.push_str("ay");
                }
            }
            println!("Pig Latin for {} is {}", s.trim(), newstr);
        }
    }
}
