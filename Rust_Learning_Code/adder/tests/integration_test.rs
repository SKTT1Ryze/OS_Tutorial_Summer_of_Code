/*
 * Learning Rust: test
 * 2020/7/2
 * hustccc
 * Manjaro
 */
use adder;
#[test]
fn it_adds_two(){
    assert_eq!(4,adder::add_two(2));
}
