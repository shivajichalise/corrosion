use crate::utils::max_value_from_hashmap::max_value_from_hashmap;
use std::collections::HashMap;

pub fn mode(data: &Vec<u32>) {
    let mut data_map = HashMap::new();

    // creating vec into hashmap
    for i in data {
        let count = data_map.entry(format!("{}", i)).or_insert(0);
        *count += 1;
    }

    if let Some(max) = max_value_from_hashmap(&data_map) {
        let modes: Vec<_> = data_map
            .iter()
            .filter_map(|(key, value)| if value == max { Some(key) } else { None })
            .collect();

        println!("Mode: {:?}", modes);
    } else {
        println!("Mode could not be calculated.");
    }
}
