/*
 * Learning Rust: functions
 * 2020/7/1
 * hustccc
 * Manjaro
 */
fn main()
{
    println!("main function");
    another_function(5,9.9);
    let x=five();
    println!("The return value of five() is: {}",x);
    println!("The return value of plus_one() is: {}",plus_one(x));
}
fn another_function(x: i32,y: f64)
{
    println!("another function");
    println!("parameter1: {}",x);
    println!("parameter2: {}",y);
}
fn five()-> i32
{
    5
}
fn plus_one(x: i32)->i32
{
    x+1
}
