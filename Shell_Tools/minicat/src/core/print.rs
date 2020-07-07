/*
 * Shell tools by Rust: mini cat
 * 2020/7/7
 * hustccc
 * Manjaro
 */
//use super::*;
pub fn show(data: String, number: bool, line_num: i32) {
    let mut lines = data.lines();
    for i in 1..line_num+1 {
        if number {
            print!("{} ",i);
        }
        match lines.next() {
            None => {},
            Some(line) => {
                println!("{}",line);
            },
        }

    }
}
