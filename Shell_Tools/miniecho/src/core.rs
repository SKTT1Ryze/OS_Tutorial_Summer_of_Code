/*
 * Shell tools by Rust: mini echo
 * 2020/7/7
 * hustccc
 * Manjaro
 */

pub mod write;
use crate::err::Error;
use std::{
    //path::PathBuf,
    fs::File,
    fs::OpenOptions,
    io::Write,
};