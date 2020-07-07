/*
 * Shell tools by Rust: mini cat
 * 2020/7/7
 * hustccc
 * Manjaro
 */

mod opt;
mod err;
mod core;
pub use self::opt::Opt;
pub use self::core::read::read_file;
pub use self::core::print::show;