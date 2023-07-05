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

fn main() {
    part1();
}
