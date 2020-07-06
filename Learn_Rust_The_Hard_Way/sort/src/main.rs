/*
 * Learn Rust The Hard Way: Sort
 * 2020/7/6
 * hustccc
 * Manjaro
 */
fn sort_maopao (data: &mut Vec<i32>) {
    let len = data.len();
    let mut i = 1;
    let mut j = 0;
    while i < len {
        while j < len-i {
            if data[j] > data[j+1] {
                let temp = data[j];
                data[j] = data[j+1];
                data[j+1] = temp;
            }
            j += 1;
        }
        i += 1;
        j = 0;
    }
}
fn sort_select (data: &mut Vec<i32>) {
    let len = data.len();
    let mut i = 0;
    let mut j;
    while i < len-1 {
        j = i+1;
        let mut minindex = i;
        while j < len {
            if data[j] < data[minindex] {
                minindex = j;
            }
            j += 1;
        }
        let temp = data[i];
        data[i] = data[minindex];
        data[minindex] = temp;
        i += 1;
    }
}

fn sort_insert(data: &mut Vec<i32>){
    let len = data.len();
    let mut i = 1;
    let mut preindex;
    let mut temp;
    while i < len {
        preindex = i-1;
        temp = data[i];
        while preindex >= 0 && data[preindex] > temp {
            data[preindex+1] = data[preindex];
            preindex -= 1;
        }
        data[preindex+1] = temp;
        i += 1;
    }
}
fn sort_shell(data: &mut Vec<i32>) {
    let len = data.len();
    let mut gap = len/2;
    let mut i;
    let mut j;
    while gap > 0 {
        i = gap;
        while i < len {
            j = i;
            let temp = data[i];
            while (j - gap) >= 0 && temp < data[j-gap] {
                data[j] = data[j-gap];
                j = j -gap;
            }
            data[j] = temp;
            i += 1;
        }
        gap = gap/2;
    }

}

fn main() {
    let mut data = vec![1,3,2,5,8,32,45,78,12,23,43];
    println!("{:?}",data);
    sort_maopao(&mut data);
    println!("maopao sort: {:?}",data);
    let mut data = vec![1,3,2,5,8,32,45,78,12,23,43];
    sort_select(&mut data);
    println!("select sort: {:?}",data);
    let mut data = vec![1,3,2,5,8,32,45,78,12,23,43];
    sort_insert(&mut data);
    println!("select sort: {:?}",data);
    let mut data = vec![1,3,2,5,8,32,45,78,12,23,43];
    sort_shell(&mut data);
    println!("select sort: {:?}",data);
}
