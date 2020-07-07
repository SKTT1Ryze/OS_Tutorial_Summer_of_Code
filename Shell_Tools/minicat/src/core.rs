/*
 * Shell tools by Rust: mini cat
 * 2020/7/7
 * hustccc
 * Manjaro
 */

pub mod read;
pub mod print;
use crate::err::Error;
use std::{
    path::PathBuf,
    fs::File,
    io::{Read},
};