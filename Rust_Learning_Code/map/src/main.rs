/*
 * Learning Rust: map
 * 2020/7/1
 * hustccc
 * Manjaro
 */
use std::collections::HashMap;
fn main(){

    let teams =vec![String::from("Blue"),String::from("Yellow")];
    let initial_scores=vec![10,50];
    let scores: HashMap<_,_>=teams.iter().zip(initial_scores.iter()).collect();
}
