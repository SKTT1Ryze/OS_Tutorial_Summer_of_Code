/*
 * Learning Rust: reference
 * 2020/7/2
 * hustccc
 * Manjaro
 */
fn main()
{
    let s1=String::from("hello");
    let len=calculate_length(&s1);
    println!("The length of '{}' is {}.",s1,len);
    let mut s=String::from("hello");
    change(&mut s);
    println!("{}",s);
}
fn calculate_length(s: &String)->usize
{
    s.len()
}
fn change(some_string: &mut String)
{
    some_string.push_str(",world");
}
