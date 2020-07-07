/*
 * Shell tools by Rust: mini echo
 * 2020/7/7
 * hustccc
 * Manjaro
 */
//use super::{Error,PathBuf,File,Write};
use super::{Error,File,Write,OpenOptions};
pub fn write_file(data: &str, filename: &str, is_append: bool) -> Result<(),Error> {
    match is_append {
        true => {
            let mut buffer = OpenOptions::new().append(true).open(filename).expect("can't open file");
            buffer.write_all("\n".as_bytes())?;
            buffer.write_all(data.as_bytes())?;
        },
        false => {
            let mut buffer = File::create(filename)?;
            buffer.write_all(data.as_bytes())?;
        }
    }
    Ok(())
}