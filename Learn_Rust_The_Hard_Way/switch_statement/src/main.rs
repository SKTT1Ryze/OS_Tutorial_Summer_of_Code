/*
 * Learn Rust The Hard Way: Switch Statement
 * 2020/7/6
 * hustccc
 * Manjaro
 */
use std::env;
fn main()  {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    let args_num = args.len();
    if args_num != 2 {
        panic!("argument error");
    }
    let letter = &args.get(1).unwrap().as_bytes();
    for (i, &item) in letter.iter().enumerate() {
        match item {
            b'a' | b'A' => {
                println!("{}: A", i);
            },
            b'e' | b'E' => {
                println!("{}: E", i);
            },
            b'i' | b'I' => {
                println!("{}: I", i);
            },
            b'o' | b'O' => {
                println!("{}: O", i);
            },
            b'u' | b'U' => {
                println!("{}: U", i);
            },
            b'y' | b'Y'=> {
                if i > 2 {
                    println!("{}: Y", i);
                }
            },
            _ => {
                println!("{}: is not a vowel", i);
            },
        }
    }
}