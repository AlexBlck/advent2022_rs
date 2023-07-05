const INPUT: &str = include_str!("../data/input.txt");

fn convert_to_priority(item: char) -> u32 {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    alphabet.iter().position(|&x| x == item).unwrap() as u32 + 1
}

fn part1() -> u32 {
    let mut priority_sum = 0;
    for line in INPUT.lines() {
        let (left, right) = line.split_at(line.chars().count() / 2);
        for item in left.chars() {
            // println!("{}", char);
            let idx = right.find(item);
            match idx {
                Some(_) => {
                    priority_sum += convert_to_priority(item);
                    break;
                },
                None => (),
            }
        }
    }
    println!("{}", priority_sum);
    priority_sum
}

fn part2() -> u32 {
    let mut sum_badges = 0;
    let mut candidates: Vec<char> = Vec::new();
    for (line_idx, line) in INPUT.lines().enumerate() {
        let line_vec = line.chars().collect::<Vec<char>>();


        if line_idx % 3 == 0 {
            for candidate in &candidates{
                sum_badges += convert_to_priority(*candidate);
            }
            candidates.clear();
            candidates.extend(line_vec);
            candidates.sort_unstable();
            candidates.dedup();
        } else {
            for item in candidates.clone().iter() {
                let idx = line_vec.iter().position(|&x| &x == item);
                match idx {
                    Some(_) => (),
                    None => candidates.retain(|&x| x != *item),
                }
            }
            
        } 
    }
    for candidate in &candidates{
        sum_badges += convert_to_priority(*candidate);
    }
    println!("{}", sum_badges);
    sum_badges
}

fn main() {
    part1();
    part2();
}
