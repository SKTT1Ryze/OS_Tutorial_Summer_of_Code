/*
 * Learning Rust: branches
 * 2020/7/1
 * hustccc
 * Manjaro
 */
use std::io;
fn main()
{
    let mut number = String::new();
    println!("input a number:");
    io::stdin().read_line(&mut number)
        .expect("Failed to read line");
    let number: u32 = number.trim().parse()
        .expect("Please type a number");
    if number < 5
    {
        println!("input < 5");
    }
    else if number < 10
    {
        println!("input>=5&&input<10");
    }
    else
    {
        println!("input >= 10");
    }
    let mut counter = number;
    let result =loop
    {
        counter-=1;
        println!("Loop: {}",counter);
        if counter<=0
        {
            break counter;
        }
    };
    println!("Loop return: {}",result);
    counter=number;
    while counter!=0 
    {
        println!("while loog: {}",counter);
        counter-=1;
    }
    println!("LIFTOFF!!!");
    let array = [1,2,3,4,5,6,7,8];
    for element in array.iter()
    {
        println!("value: {}",element);
    }
    for i in (1..5).rev()
    {
        println!("{}!",i);
    }
    println!("LIFTOFF!!!");
}
