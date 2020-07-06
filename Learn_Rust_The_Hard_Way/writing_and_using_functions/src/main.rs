/*
 * Learn Rust The Hard Way: Writing and Using Functions
 * 2020/7/6
 * hustccc
 * Manjaro
 */
 use std::env;
 fn print_arguments(args: &Vec<String>) {
     for word in args {
         print_letters(&word[..]);
     }
 }

 fn print_letters(word: &str) {
    print!("{} ",word);
 }

 fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    let args_num = args.len();
    if args_num != 2 {
        panic!("argument error");
    }
    print_arguments(&args);
 }