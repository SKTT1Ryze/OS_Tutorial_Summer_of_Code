/*
 * Learn Rust The Hard Way: Arrays and Strings
 * 2020/7/6
 * hustccc
 * Manjaro
 */

fn main () {
    let mut numbers = [0; 4];
    let mut name = String::from("a");
    //first, print them out raw
    println!("numbers: {}, {}, {}, {}", numbers[0], numbers[1], numbers[2], numbers[3]);
    
    println!("name: {}",name);

    //setup the numbers
    numbers[0] = 1;
    numbers[1] = 2;
    numbers[2] = 3;
    numbers[3] = 4;

    //setup the name
    name = "Zed".to_string();

    //then print them out initialized
    println!("numbers: {}, {}, {}, {}", numbers[0], numbers[1], numbers[2], numbers[3]);
    
    println!("name: {}",name);

    let another =name;

    println!("another: {}", another);
}
