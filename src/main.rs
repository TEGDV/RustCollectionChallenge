use std::collections::HashMap;

fn main() {
    let mut my_list = vec![
        2.0, 4.0, 6.0, 5.0, 4.1, 4.1, 4.1, 4.1, 4.1, 1.0, 1.0, 1.0, 2.0, 2.0, 4.1, 4.1, 5.0, 6.0,
        7.0, 8.0, 8.0, 9.0,
    ];
    let mean: f64 = my_list.iter().sum();
    let mean: f64 = mean / 2.0;
    my_list.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median_index: usize = my_list.len() / 2;
    let mut map = HashMap::new();
    let mut max_value = 0;
    let mut max_key = String::new();
    for float in &my_list {
        let count = map.entry(float.to_string()).or_insert(0);
        *count += 1;
        if *count > max_value {
            max_value = *count;
            max_key = format!("{}", float);
        }
    }

    println!(
        "The mean is {} the median is {}, and the most repetead value is {} with {} times",
        mean, my_list[median_index], max_key, map[&max_key]
    );
}
