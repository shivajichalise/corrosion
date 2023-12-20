use rand::Rng;
mod statistics;
mod utils;
use statistics::{mean::mean, median::median, mode::mode};

fn main() {
    let number_of_data: u32 = 9;
    let mut data: Vec<u32> = Vec::new();

    for _ in 0..number_of_data {
        data.push(rand::thread_rng().gen_range(1..=11));
    }

    println!("Data is: {:?}", data);
    println!();

    mean(&data);

    median(&mut data);

    mode(&data);
}
