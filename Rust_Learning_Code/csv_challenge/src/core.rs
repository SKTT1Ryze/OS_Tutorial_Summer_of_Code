/*
 * <dao of Rust> chapter 10 code
 * 2020/7/2
 * hustccc
 * Manjaro
 */
pub mod read;
pub mod write;
use crate::err::Error;
use std::{
    path::PathBuf,
    fs::File,
    io::{Read,Write},
};