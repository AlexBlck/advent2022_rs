const INPUT: &str = include_str!("../data/input.txt");


fn part1(input: &str) -> u32 {
    for idx in 4..input.len() {
        let slice:&str = &input[idx-4..idx];
        // number of unique characters in slice
        let mut unique_chars: Vec<char> = slice.chars().collect();
        unique_chars.sort();
        unique_chars.dedup();
        if unique_chars.len() == 4 {
            return idx as u32;
        }
    }
    0
}



fn main() {
    let part1_answer: u32 = part1(INPUT);
    println!("Part 1 answer: {}", part1_answer);
    // part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_case1() {
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), (5));
    }
    #[test]
    fn test_part1_case2() {
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), (6));
    } 
    #[test]
    fn test_part1_case3() {
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), (10));
    }
    #[test]
    fn test_part1_case4() {
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), (11));
    }
}
