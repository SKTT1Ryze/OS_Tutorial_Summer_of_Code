/*
 * Shell tools by Rust: mini copy
 * 2020/7/7
 * hustccc
 * Manjaro
 */
use structopt::StructOpt;
use minicopy::{
    Opt,
    {read_file,write_file},
};
use std::path::PathBuf;
use std::process;

fn main() {
    let opt = Opt::from_args();
    let source_name = PathBuf::from(opt.source);
    let target_name = &opt.target;
    let source_data = match read_file(source_name) {
        Ok(data) => {data},
        Err(e) => {
            println!("main error: {:?}",e);
            process::exit(1);
        }
    };
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
    match write_file(&source_data,&target_name,is_append) {
        Ok(_) => {
            println!("write sucess");
        },
        Err(e) => {
            println!("main error: {:?}",e);
            process::exit(1);
        }
    }
}