/*
 * Shell tools by Rust: mini copy
 * 2020/7/7
 * hustccc
 * Manjaro
 */

use structopt_derive::*;
#[derive(StructOpt,Debug)]
#[structopt(name = "minicopy" , about = "Usage")]
pub struct Opt{
    #[structopt(help = "source")]
    pub source: String,
    #[structopt(help = "target")]
    pub target: String,
    #[structopt(short = "a", long = "append")]
    pub append: Option<String>,
}