use std::collections::HashSet;

fn main() {
    let input: Vec<i32> = include_str!("../../data/day1.txt").lines().map(|num| num.parse::<i32>().unwrap()).collect();

    let sum: i32 = input.iter().sum();
    println!("sum: {:?}", sum);

    let mut seen_magnitudes = HashSet::new();
    seen_magnitudes.insert(0);

    let mut sum_loop: i32 = 0;

    'outer: for _ in 0..1000 {
        for (i, item) in input.iter().enumerate() {

            sum_loop += item;
            if i != 0 && seen_magnitudes.contains(&sum_loop) {
                println!("second time we see: {}", sum_loop);
                break 'outer;
            }
            seen_magnitudes.insert(sum_loop);
        }
    }
}
