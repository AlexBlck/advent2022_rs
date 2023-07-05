const INPUT: &str = include_str!("../data/input.txt");

fn parse_line(line: &str) -> (u32, u32, u32, u32) {
    // <int>-<int>,<int>-<int>
    // start1-end1,start2-end2
    let (sections1, sections2) = line.split_once(",").unwrap();
    let (start1, end1) = sections1.split_once("-").unwrap();
    let (start2, end2) = sections2.split_once("-").unwrap();

    // Convert to u32
    let start1 = start1.parse::<u32>().unwrap();
    let end1 = end1.parse::<u32>().unwrap();
    let start2 = start2.parse::<u32>().unwrap();
    let end2 = end2.parse::<u32>().unwrap();

    (start1, end1, start2, end2)
}

fn part1() -> u32 {
    let mut contained_pairs = 0;
    for line in INPUT.lines(){
        let (start1, end1, start2, end2) = parse_line(line);
        if start1 <= start2 && end1 >= end2 {
            contained_pairs += 1;
        } else if start2 <= start1 && end2 >= end1 {
            contained_pairs += 1;
        }
    } 
    println!("{}", contained_pairs);
    contained_pairs
}

fn part2() -> u32 {
    let mut overlapping_assignments = 0;
        for line in INPUT.lines(){
            let (start1, end1, start2, end2) = parse_line(line);
            if start2 <= end1 && start2 >= start1 {
                // ..start1..end1.. 
                // ....start2..end2..
                overlapping_assignments += 1;
            } else if start1 <= end2 && start1 >= start2 {
                // ....start1..end1..
                // ..start2..end2..
                overlapping_assignments += 1;
            }
        } 
        println!("{}", overlapping_assignments);
        overlapping_assignments
}

fn main() {
    part1();
    part2();
}
