use std::cmp::max;

const INPUT: &str = include_str!("../data/input.txt");

fn part1() -> i32 {
    let mut max_calories = 0;
    let mut cur_calories = 0;

    for line in INPUT.lines() {
        if line.len() == 0 {
            max_calories = max(max_calories, cur_calories);
            cur_calories = 0;
        } else {
            cur_calories += line.parse::<i32>().unwrap();
        }
    }

    max_calories
}

fn part2() -> i32 {
    let mut max_calories = vec![0; 3];
    let mut cur_calories = 0;

    for line in INPUT.lines() {
        if line.len() == 0 {
            if cur_calories > max_calories[0] {
                max_calories[0] = cur_calories;
                max_calories.sort();
            }
            cur_calories = 0;
        } else {
            cur_calories += line.parse::<i32>().unwrap();
        }
    }

    max_calories.iter().sum()
}

fn main() {
    println!("Max calories: {}", part1());
    println!("Sum of top-3 calories: {}", part2());
}
