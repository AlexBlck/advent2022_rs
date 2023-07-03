use std::collections::HashMap;
use itertools::Itertools;
const INPUT: &str = include_str!("../data/input.txt");


#[derive(Clone)]
enum Symbol {
    Rock,
    Paper,
    Scissors,
    Win,
    Lose,
    Draw,
}

fn part1(symbols: &HashMap<&str, Symbol>) -> u32 {
    let mut opponent: &str;
    let mut me: &str;
    let mut score: u32 = 0;

    for line in INPUT.lines() {
        (opponent, me) = line.split_whitespace().next_tuple().unwrap();
        let me_symbol = symbols.get(&me).unwrap();
        let opponent_symbol = symbols.get(&opponent).unwrap();

        match me_symbol {
            Symbol::Rock => score += 1,
            Symbol::Paper => score += 2,
            Symbol::Scissors => score += 3,
            _ => (),
        }

        match (me_symbol, opponent_symbol) {
            (Symbol::Rock, Symbol::Scissors) => score += 6,
            (Symbol::Rock, Symbol::Paper) => score += 0,
            (Symbol::Paper, Symbol::Rock) => score += 6,
            (Symbol::Paper, Symbol::Scissors) => score += 0,
            (Symbol::Scissors, Symbol::Paper) => score += 6,
            (Symbol::Scissors, Symbol::Rock) => score += 0,
            _ => score += 3,
        }
    }

    score
}


fn part2(symbols: &HashMap<&str, Symbol>) -> u32 {
    let mut opponent: &str;
    let mut me: &str;
    let mut score: u32 = 0;

    for line in INPUT.lines() {
        (opponent, me) = line.split_whitespace().next_tuple().unwrap();
        let me_result = symbols.get(&me).unwrap();
        let opponent_symbol = symbols.get(&opponent).unwrap();
        let me_symbol: Symbol;

        match (me_result, opponent_symbol) {
            (Symbol::Win, Symbol::Scissors) => {
                me_symbol = Symbol::Rock;
                score += 6;
            },
            (Symbol::Win, Symbol::Paper) => {
                me_symbol = Symbol::Scissors;
                score += 6;
            },
            (Symbol::Win, Symbol::Rock) => {
                me_symbol = Symbol::Paper;
                score += 6;
            },
            (Symbol::Lose, Symbol::Scissors) => {
                me_symbol = Symbol::Paper;
                score += 0;
            },
            (Symbol::Lose, Symbol::Paper) => {
                me_symbol = Symbol::Rock;
                score += 0;
            },
            (Symbol::Lose, Symbol::Rock) => {
                me_symbol = Symbol::Scissors;
                score += 0;
            },
            _ => {
                me_symbol = opponent_symbol.to_owned();
                score += 3;
            }
        }

        match me_symbol {
            Symbol::Rock => score += 1,
            Symbol::Paper => score += 2,
            Symbol::Scissors => score += 3,
            _ => (),
        }


    }

    score
}

fn main() {

	let symbols = HashMap::from([
	    ("A",Symbol::Rock),
	    ("B", Symbol::Paper), 
	    ("C", Symbol::Scissors),
	    ("X", Symbol::Rock), 
	    ("Y", Symbol::Paper), 
	    ("Z", Symbol::Scissors),
	]);

	println!("Score 1: {}", part1(&symbols));

	let symbols = HashMap::from([
	    ("A", Symbol::Rock),
	    ("B", Symbol::Paper), 
	    ("C", Symbol::Scissors),
	    ("X", Symbol::Lose), 
        ("Y", Symbol::Draw), 
        ("Z", Symbol::Win),
	]);

	println!("Score 2: {}", part2(&symbols));

}
