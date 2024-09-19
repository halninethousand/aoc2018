use itertools::Itertools;

fn main() {
    let input: Vec<&str> = include_str!("../../data/day2.txt").lines().map(|str| str.trim()).collect();

    let mut all_twos = 0; 
    let mut all_threes = 0; 


    for item in &input {
        let counts = item.chars().counts();
        if counts.values().any(|&count| count == 2) {
            all_twos += 1;
        }
        if counts.values().any(|&count| count == 3) {
            all_threes += 1;
        }
    }


    for combination in input.iter().combinations(2) {
        if count_differing_chars(combination[0], combination[1]) == 1 {
            println!("common letters: {}", find_common_letters(combination[0], combination[1]));
            break;
        }
    }

    println!("cheskum: {}", all_twos * all_threes);
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
