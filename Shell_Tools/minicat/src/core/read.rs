/*
 * Shell tools by Rust: mini cat
 * 2020/7/7
 * hustccc
 * Manjaro
 */
use super::{Error,PathBuf,File,Read};
pub fn read_file(file: PathBuf) -> Result<String, Error> {
    let temp_file = read(file)?;
    Ok(temp_file)
}

fn read(path: PathBuf) -> Result<String, Error> {
    let mut buffer = String::new();
    let mut temp_file = open(path)?;
    temp_file.read_to_string(&mut buffer)?;
    match buffer.is_empty() {
        true => {
            return Err("input file missing")?
        },
        false => {
            return Ok(buffer);
        },
    }
}

fn open(path: PathBuf) -> Result<File, Error> {
    let temp_file = File::open(path)?;
    Ok(temp_file)
}
