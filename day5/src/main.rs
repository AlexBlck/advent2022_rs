const INPUT: &str = include_str!("../data/input.txt");

fn parse_instruction_line(line: &str) -> (usize, usize, usize) {
    // move <int> from <int> to <int>
    let line_contents = line.split_whitespace().collect::<Vec<&str>>();
    let qty = line_contents[1];
    let source_container = line_contents[3];
    let dest_container = line_contents[5];
    // Convert to u32
    let qty = qty.parse::<usize>().unwrap();
    let source_container = source_container.parse::<usize>().unwrap();
    let dest_container = dest_container.parse::<usize>().unwrap();

    (qty, source_container, dest_container)
}

fn part1() -> () {
    let mut containers: Vec<Vec<&str>> = Vec::new();
    // nth crate contents: 4n + 1
    // line length: 4n - 1
    let mut reading_containers = true;
    for line in INPUT.lines(){
        if reading_containers {
            if line == "" {
                reading_containers = false;
                for container in containers.iter_mut() {
                    container.pop();
                    container.reverse();
                }
            } else {
                let num_containers = (line.len() + 1) / 4; 
                for n in 0..num_containers {
                    match containers.get_mut(n){
                        Some(x) => {
                            let crate_contents = line.get(n*4+1..n*4+2).unwrap();
                            if crate_contents != " " {
                                x.push(crate_contents);
                            }
                        }
                        None => {
                            let crate_contents = line.get(n*4+1..n*4+2).unwrap();
                            if crate_contents != " " {
                                containers.push(vec![crate_contents]);
                            } else {
                                containers.push(vec![]);
                            }
                        }
                    }
                }
            }
        } else {
            let (qty, source_container_id, dest_container_id) = parse_instruction_line(line);
            // cast qty from u32 to usize
            for _ in 0..qty {
                let source_container = containers.get_mut(source_container_id - 1).unwrap();
                let contents = source_container.pop();
                let dest_container = containers.get_mut(dest_container_id - 1).unwrap();
                dest_container.extend(contents);
            }
        }
    } 
    let mut final_answer = String::new();
    for container in containers.iter_mut() {
        final_answer.push_str(container.pop().unwrap());
    }
    println!("Final answer: {}", final_answer);
}

fn part2() -> () {
}

fn main() {
    part1();
    part2();
}
