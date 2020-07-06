/*
 * Learn Rust The Hard Way: Array Of Strings and Looping
 * 2020/7/6
 * hustccc
 * Manjaro
 */

use std::env;

fn main() {

    //go through each string in argv
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    //let's make our own array of strings
    let str_vec: Vec<String> = vec![
        "California".to_string(),
        "Oregon".to_string(),
        "Washington".to_string(),
        "Texas".to_string()
    ];
    println!("string vector: {:?}", str_vec);
}