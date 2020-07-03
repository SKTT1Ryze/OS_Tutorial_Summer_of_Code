/*
 * <dao of Rust> chapter 10 code
 * 2020/7/2
 * hustccc
 * Manjaro
 */
#![feature(test)]
extern crate test;
use test::Bencher;
use std::path::PathBuf;
use csv_challenge::{
    Opt,
    {load_csv,write_csv},
    replace_column,
};
#[bench]
fn bench_read_100times(b: &mut Bencher){
    b.iter(||{
        let n = test::black_box(100);
        (0..n).fold(0, |_,_|{test_load_csv();0})
    });
}
fn test_load_csv(){
    let filename=PathBuf::from("./input/challenge.csv");
    load_csv(filename);
}