/*
 * Shell tools by Rust: mini echo
 * 2020/7/7
 * hustccc
 * Manjaro
 */

mod opt;
mod err;
mod core;
pub use self::opt::Opt;
pub use self::core::write::write_file;