const INPUT: &str = include_str!("../data/input.txt");


fn find_first_n_distinct(input: &str, n: usize) -> u32 {
    for idx in n..input.len() {
        let slice:&str = &input[idx-n..idx];
        let mut unique_chars: Vec<char> = slice.chars().collect();
        unique_chars.sort();
        unique_chars.dedup();
        if unique_chars.len() == n {
            return idx as u32;
        }
    }
    0
}

fn main() {
    let part1_answer: u32 = find_first_n_distinct(INPUT, 4);
    let part2_answer: u32 = find_first_n_distinct(INPUT, 14);
    println!("Part 1 answer: {}", part1_answer);
    println!("Part 2 answer: {}", part2_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_case1() {
        assert_eq!(find_first_n_distinct("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), (5));
    }
    #[test]
    fn test_part1_case2() {
        assert_eq!(find_first_n_distinct("nppdvjthqldpwncqszvftbrmjlhg", 4), (6));
    } 
    #[test]
    fn test_part1_case3() {
        assert_eq!(find_first_n_distinct("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), (10));
    }
    #[test]
    fn test_part1_case4() {
        assert_eq!(find_first_n_distinct("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), (11));
    }
    
    #[test]
    fn test_part2_case0() {
        assert_eq!(find_first_n_distinct("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), (19));
    }
    #[test]
    fn test_part2_case1() {
        assert_eq!(find_first_n_distinct("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), (23));
    }
    #[test]
    fn test_part2_case2() {
        assert_eq!(find_first_n_distinct("nppdvjthqldpwncqszvftbrmjlhg", 14), (23));
    } 
    #[test]
    fn test_part2_case3() {
        assert_eq!(find_first_n_distinct("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), (29));
    }
    #[test]
    fn test_part2_case4() {
        assert_eq!(find_first_n_distinct("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), (26));
    }
}
