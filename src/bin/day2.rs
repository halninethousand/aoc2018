use itertools::Itertools;

fn main() {
    let input: Vec<&str> = include_str!("../../data/day2.txt").lines().map(|str| str.trim()).collect();

    let mut twos = 0; 
    let mut threes = 0; 


    for item in &input {
        let counts = item.chars().counts();
        if counts.values().any(|&count| count == 2) {
            twos += 1;
        }
        if counts.values().any(|&count| count == 3) {
            threes += 1;
        }
    }

    if let Some(combination) = input.iter().combinations(2).find(|combo| count_differing_chars(combo[0], combo[1]) == 1) {
        println!("common letters: {}", find_common_letters(combination[0], combination[1]));
    }

    println!("cheskum: {}", twos * threes);
}

fn count_differing_chars(s1: &str, s2: &str) -> usize {
    s1.chars()
        .zip(s2.chars())
        .filter(|&(c1, c2)| c1 != c2)
        .count()
}

fn find_common_letters(s1: &str, s2: &str) -> String {
    s1.chars()
        .zip(s2.chars())
        .filter(|&(c1, c2)| c1 == c2)
        .map(|(c, _)| c)
        .collect()
}
