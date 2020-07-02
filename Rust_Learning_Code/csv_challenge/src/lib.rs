/*
 * <dao of Rust> chapter 10 code
 * 2020/7/2
 * hustccc
 * Manjaro
 */
mod opt;
mod err;
mod core;
pub use self::opt::Opt;
pub use self::core::{
    read::{load_csv,write_csv},
    write::replace_column,
};