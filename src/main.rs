fn sum(list: &[u32]) -> Option<u32> {
    let count = list.iter().sum();

    if count > 10 {
        None
    } else {
        Some(count)
    }
}

fn main() {
    // 溢出
    let num_list: [u32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", sum(&num_list));

    // 没溢出
    let num_list: [u32; 4] = [1, 2, 3, 4];
    println!("{:?}", sum(&num_list))
}

