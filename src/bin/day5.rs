fn main() {
    let input: Vec<char> = include_str!("../../data/day5.txt").trim().chars().collect();
    
    let alphabet = (b'a'..=b'z').map(|c| c as char);
    let mut shortest_length = usize::MAX;
    let mut remove_char: Option<char> = None;

    // part 1
    println!("reacted polymer length: {}", polymer(input.clone()).len());

    // part 2
    for unit in alphabet {
        let filtered_input: Vec<char> = input
            .iter()
            .cloned()
            .filter(|&c| c != unit && c != unit.to_ascii_uppercase())
            .collect();

        let reacted_length = polymer(filtered_input).len();

        if reacted_length < shortest_length {
            shortest_length = reacted_length;
            remove_char = Some(unit);
        }
    }

    println!("shortest polymer length: {} for {}", shortest_length, remove_char.unwrap());
}

fn polymer(polymer: Vec<char>) -> String {
    let mut stack = Vec::new();

    for c in polymer {
        if let Some(&last) = stack.last() {
            if c != last && c.to_ascii_lowercase() == last.to_ascii_lowercase() {
                stack.pop();
            } else {
                stack.push(c);
            }
        } else {
            stack.push(c);
        }
    }

    stack.into_iter().collect()
}
