/*
 * <dao of Rust> chapter 10 code
 * 2020/7/2
 * hustccc
 * Manjaro
 */

 use structopt_derive::*;
 #[derive(StructOpt,Debug)]
 #[structopt(name="csv_challenge",about="Usage")]
 pub struct Opt{
     #[structopt(help="Input file")]
     pub input: String,
     #[structopt(help="Column Name")]
     pub column_name: String,
     #[structopt(help="Replacement Column Name")]
     pub replacement: String,
     #[structopt(help="Output file, stdout if not present")]
     pub output: Option<String>,
 }
 // USAGE:
 // csv_challenge [FLAGS] <input> <column_name> <replacement> [output]