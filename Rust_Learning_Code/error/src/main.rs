/*
 * Learning Rust: error
 * 2020/7/1
 * hustccc
 * Manjaro
 */
use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
fn main(){
    let f=File::open("hello.txt");
    let f=match f{
        Ok(file)=>file,
        Err(error)=>match error.kind(){
            ErrorKind::NotFound=>match File::create("hello.txt"){
                Ok(fc)=>fc,
                Err(e)=>panic!("Problem creating the file: {:?}",e),
            },
            other_error=>panic!("Problem creating the file: {:?}",other_error),
        },
    };
}
fn read_username_from_file()->Result<String,io::Error>{
    let mut s =String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}