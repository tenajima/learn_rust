use std::collections::HashMap;
fn practice_1() {
    let mut map = HashMap::new();
    let mut v = vec![1, 1, 1, 2, 3, 4, 5, 6, 6, 6, 6];
    v.sort();
    let median = v[v.len() / 2];
    // let modian = 0
    let mut sum = 0;
    let mut modian = &v[0];
    let mut modian_counter = 0;
    for i in &v {
        sum += i;
        let count = map.entry(i).or_insert(0);
        *count += 1;
        if *count > modian_counter {
            modian = &i;
            modian_counter = *count;
        }
    }
    println!(
        "mean: {}, midian: {}, modian: {}",
        sum as f64 / v.len() as f64,
        median,
        *modian
    );
    println!("{:?}", map)
}
fn main() {
    practice_1();
}
