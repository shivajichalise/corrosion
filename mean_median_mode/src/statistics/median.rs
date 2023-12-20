pub fn median(data: &mut Vec<u32>) {
    data.sort();

    let median;

    if data.len() % 2 == 0 {
        let mid = data.len() / 2;

        let mid_second = data.len() / 2 + 1;

        median = (data[mid] + data[mid_second]) / 2;
    } else {
        let mid = data.len() / 2;

        median = data[mid];
    }

    println!("Median is: {median}");
}
