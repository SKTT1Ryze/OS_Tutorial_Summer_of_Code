/*
 * Learning Rust: tuple
 * 2020/7/1
 * hustccc
 * Manjaro
 */
fn main() {
    let tup:(i32,f64,u8)=(500,6.4,1);
    let (x,y,z)=tup;
    println!("The value of x,y,z is {},{},{}",x,y,z);
    println!("The value of tup is ({},{},{})",tup.0,tup.1,tup.2);
}
