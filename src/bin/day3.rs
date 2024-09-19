fn main () {
    let input: Vec<&str> = include_str!("../../data/day3.txt").lines().collect();
    let mut grid: Vec<Vec<u16>> = vec![vec![0; 1000]; 1000];
    
    // part1 populate grid
    for item in input.clone() {
        let patch: Vec<&str> = item.split_whitespace().collect();
        let location: Vec<u16> = drop_last_char(patch[2]).split(',').map(|n| n.parse::<u16>().unwrap()).collect();
        let (x, y) = (location[0], location[1]);
        let size: Vec<u16> = patch[3].split('x').map(|n| n.parse::<u16>().unwrap()).collect();
        let (size_x, size_y) = (size[0], size[1]);
        
        for eks in y..y+size_y {
            for why in x..x+size_x {
                grid[why as usize][eks as usize] += 1;
            }
        }
    }

    let count = grid.iter().flatten().filter(|&&num| num > 1).count();
    println!("count of squares within 2 or more patches: {}", count);
    
    // part2 non overlapping area
    for item in input {
        let patch: Vec<&str> = item.split_whitespace().collect();
        let location: Vec<u16> = drop_last_char(patch[2]).split(',').map(|n| n.parse::<u16>().unwrap()).collect();
        let (x, y) = (location[0], location[1]);
        let size: Vec<u16> = patch[3].split('x').map(|n| n.parse::<u16>().unwrap()).collect();
        let (size_x, size_y) = (size[0], size[1]);
        let mut overlap = false; 

        for eks in y..y+size_y {
            for why in x..x+size_x {
                if grid[why as usize][eks as usize] > 1 {
                    overlap = true;
                }
            }
        }

        if overlap == false {
            let id: &str = &patch[0][1..];         
            println!("non-overlapping patch ID: {id}");
        }

    }
    // grid_print(&grid);
}

fn grid_print(grid: &[Vec<u16>]) {
    for item in grid {
        for num in item {
            print!("{num}");
        }
        println!();
    } 
}

fn drop_last_char(s: &str) -> String {
    let mut chars = s.chars();
    chars.next_back();
    let result: String = chars.collect();
    result
}
