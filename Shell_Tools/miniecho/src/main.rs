/*
 * Shell tools by Rust: mini echo
 * 2020/7/7
 * hustccc
 * Manjaro
 */

use structopt::StructOpt;
use miniecho::{
    Opt,
    write_file,
};
//use std::path::PathBuf;
use std::process;

fn main() {
    let opt = Opt::from_args();
    let is_append: bool;
    match &opt.append {
        None => {
            is_append = false;
        },
        Some(str) => {
            match &str[..] {
                "yes" => {
                    is_append = true;
                },
                "no" => {
                    is_append = false;
                },
                _ => {
                    println!("Please input yes or no");
                    process::exit(1);
                }
            }
        }
    }
    match &opt.file {
        None => {
            println!("no file");
            println!("content: {}",&opt.content);
        },
        Some(filename) => {
            write_file(&opt.content,filename, is_append).unwrap();
            println!("write in file: {}", filename);
        },
    }
}