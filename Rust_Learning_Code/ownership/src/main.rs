/*
 * Learning Rust: ownership
 * 2020/7/2
 * hustccc
 * Manjaro
 */
fn main()
{
    let s=String::from("hello");    //s into space
    takes_ownership(s);             //s move into takes_ownership()
                                    //s become invalid here
    let x=5;                        //x into space
    makes_copy(x);                  //x move into takes_copy()
                                    //x remain valid because i32 is copy
    let s1=gives_ownership();
    let s2=String::from("hello");
    let s3=takes_and_gives_back(s2);
}

fn takes_ownership(some_string: String)
{
    //some_string into space
    println!("{}",some_string);
}//drop

fn makes_copy(some_integer: i32)
{
    //some_integer into space
    println!("{}",some_integer);
}//no drop

fn gives_ownership()->String
{
    //some_string into space
    let some_string=String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String)->String
{
    a_string
}
