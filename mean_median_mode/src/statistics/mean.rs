pub fn mean(data: &Vec<u32>) {
    let len: u32 = data
        .len()
        .try_into()
        .expect("Failed to parse usize into u32");

    let mut sum = 0;

    for v in data {
        sum += v;
    }

    let mean = sum / len;
    println!("Mean is: {mean }");
}
