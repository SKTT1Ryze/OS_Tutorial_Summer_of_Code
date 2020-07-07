/*
 * Shell tools by Rust: mini cat
 * 2020/7/7
 * hustccc
 * Manjaro
 */

use structopt::StructOpt;
use minicat::{
    Opt,
    read_file,
    show,
};
use std::path::PathBuf;
use std::process;

fn main() {
    //println!("test");
    let opt = Opt::from_args();
    let filename = PathBuf::from(&opt.file);
    let file_data = match read_file(filename) {
        Ok(data) => {data},
        Err(e) => {
            println!("main error: {:?}", e);
            process::exit(1);
        },
    };
    //println!("file name: {}", &opt.file);
    
    let is_show_number: bool;
    let show_line_number: i32;
    match &opt.number {
        None => {
            is_show_number = false;
        },
        Some(num) => {
            let temp_str = &num[..];
            match temp_str {
                "yes" => {
                    //println!("show number");
                    is_show_number = true;
                },
                "no" => {
                    //println!("don't show number");
                    is_show_number = false;
                },
                _ => {
                    println!("-n option: please input yes or no");
                    process::exit(1);
                },
            }
        },
    }
    match &opt.line {
        None => {
            //println!("line default");
            show_line_number = file_data.lines().count() as i32;
        },
        Some(line) => {
            //println!("line: {}", line);
            show_line_number = match *line >= 0 {
                true => *line,
                false => {
                    println!("-l option: please input positive integer");
                    process::exit(1);
                },
            };
        }
    }
    show(file_data,is_show_number,show_line_number);
}
