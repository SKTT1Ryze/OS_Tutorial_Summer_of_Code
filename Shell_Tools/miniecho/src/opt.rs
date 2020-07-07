/*
 * Shell tools by Rust: mini echo
 * 2020/7/7
 * hustccc
 * Manjaro
 */

use structopt_derive::*;
#[derive(StructOpt,Debug)]
#[structopt(name = "miniecho" , about = "Usage")]
pub struct Opt{
    #[structopt(help = "content")]
    pub content: String,
    #[structopt(short = "f", long = "file")]
    pub file: Option<String>,
    #[structopt(short = "a", long = "append")]
    pub append: Option<String>,
}