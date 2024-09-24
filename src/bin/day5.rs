fn main() {
    let mut input: Vec<char> = include_str!("../../data/day5_short.txt").trim().chars().collect(); 
    println!("{input:?}");

    let mut annihilated = 0;
    let mut i = 0;
    let mut j = 1;
    let original_length = input.len();

    loop {
        let length = input.len();

        if i >= length || j >= length {
            break;
        }

        let char1 = input[i];
        let char2 = input[j];
        let first = char1 as u8;
        let second = char2 as u8;
        


        if char1.is_uppercase() && char2.is_lowercase() && (second as i8 - first as i8 == 32) {
            // println!("annihilated: {} -> {}, {} -> {}", char1, first, char2, second); 
            input.remove(j);
            input.remove(i);
            i = 0;
            j = 1;
            annihilated += 2;
        } else if char1.is_lowercase() && char2.is_uppercase() && (first as i8 - second as i8 == 32) {
            // println!("annihilated: {} -> {}, {} -> {}", char1, first, char2, second); 
            input.remove(j);
            input.remove(i);
            i = 0;
            j = 1;
            annihilated += 2;
        } else {
            i += 1;
            j += 1;
        }


    }

    println!("elements left: {}", original_length - annihilated);


}
