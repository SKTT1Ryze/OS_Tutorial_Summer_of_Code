/*
 * Learning Rust: macro
 * 2020/7/2
 * hustccc
 * Manjaro
 */
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;
fn main(){
    Pancakes::hello_macro();
}
