use std::collections::HashMap;
use itertools::Itertools;
const INPUT: &str = include_str!("../data/input.txt");

#[derive(Clone)]
enum Symbol {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone)]
enum Outcome{
    Win,
    Lose,
    Draw,
}

struct Matchup {
    opponent_symbol: Symbol,
    outcome: Outcome,
}

impl Matchup {
    fn calculate_points(&self) -> u32 {
        let outcome_points = match self.outcome {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        };
    
        let symbol_points = match self.choose_symbol() {
            Symbol::Rock => 1,
            Symbol::Paper => 2,
            Symbol::Scissors => 3,
        };
        
        outcome_points + symbol_points
    
    }

    fn choose_symbol(&self) -> Symbol {
        match self.outcome {
            Outcome::Win => match self.opponent_symbol {
                Symbol::Rock => Symbol::Paper,
                Symbol::Paper => Symbol::Scissors,
                Symbol::Scissors => Symbol::Rock,
            },
            Outcome::Draw => self.opponent_symbol.clone(),
            Outcome::Lose => match self.opponent_symbol {
                Symbol::Rock => Symbol::Scissors,
                Symbol::Paper => Symbol::Rock,
                Symbol::Scissors => Symbol::Paper,
            }
        }
    }
}



fn part2(symbols: &HashMap<&str, Symbol>, outcomes: &HashMap<&str, Outcome>) -> u32 {
    let mut opponent: &str;
    let mut me: &str;
    let mut score: u32 = 0;

    for line in INPUT.lines() {
        (opponent, me) = line.split_whitespace().next_tuple().unwrap();
        let me_result = outcomes.get(&me).unwrap();
        let opponent_symbol = symbols.get(&opponent).unwrap();
        
        let matchup = Matchup {
            opponent_symbol: opponent_symbol.to_owned(),
            outcome: me_result.to_owned(),
        };

        score += matchup.calculate_points();

    }

    score
}

fn main() {

	let symbols = HashMap::from([
	    ("A", Symbol::Rock),
	    ("B", Symbol::Paper), 
	    ("C", Symbol::Scissors),
	]);

	let outcomes = HashMap::from([
        ("Z", Outcome::Win),
        ("Y", Outcome::Draw), 
        ("X", Outcome::Lose),
    ]);

	println!("Score 2: {}", part2(&symbols, &outcomes));

}
