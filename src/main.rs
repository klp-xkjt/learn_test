fn main() {
    let a: usize = 10;
    let list: [i32; 4] = [0, 1, 2, 3];
    let list_first: i32 = match safe_get(&list, a) {
        Ok(re) => re,
        Err(error) => panic!("{error:?}")
    };
    println!("{list_first}");
}

fn safe_get(list: &[i32], index: usize) -> Result<i32, String> {
    if index < list.len() {
        Ok(list[index])
    } else {
        Err(format!("索引 {} 越界了！数组长度只有 {}", index, list.len()))
    }
}