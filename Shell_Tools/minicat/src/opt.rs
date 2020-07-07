/*
 * Shell tools by Rust: mini cat
 * 2020/7/7
 * hustccc
 * Manjaro
 */

use structopt_derive::*;
#[derive(StructOpt,Debug)]
#[structopt(name = "minicat" , about = "Usage")]
pub struct Opt{
    //#[structopt(help = "FILE")]
    #[structopt(short = "f", long = "file")]
    pub file: String,
    #[structopt(short = "l", long = "line")]
    pub line: Option<i32>,
    #[structopt(short = "n", long = "number")]
    pub number: Option<String>,
}
// USAGE:
// minicat [OPTION] [LINE] [FILE]