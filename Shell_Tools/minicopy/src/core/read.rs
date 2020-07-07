/*
 * Shell tools by Rust: mini copy
 * 2020/7/7
 * hustccc
 * Manjaro
 */
use super::{Error,PathBuf,File,Read};

pub fn read_file(source_name: PathBuf) -> Result<String, Error> {
    let file = read(source_name)?;
    Ok(file)
}

fn read(path: PathBuf)->Result<String,Error>{
    let mut buffer=String::new();
    let mut file = open(path)?;
    file.read_to_string(&mut buffer)?;
    if buffer.is_empty(){
        return Err("input file missing")?
    }
    Ok(buffer)
}

fn open(path: PathBuf)->Result<File,Error>{
    let file = File::open(path)?;
    Ok(file)
}
