#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    const MAP: [(Move, Move); 3] = {
        use Move::*;
        [(Rock, Scissors), (Paper, Rock), (Scissors, Paper)]
    };

    fn beats(&self) -> Self {
        Move::MAP.iter().find(|(mv1, _)| mv1 == self).unwrap().1
    }

    fn loses_to(&self) -> Self {
        Move::MAP.iter().find(|(_, mv2)| mv2 == self).unwrap().0
    }
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        use Move::*;

        match s {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!("Invalid string"),
        }
    }
}

enum Round {
    Lose,
    Draw,
    Win,
}

impl From<&str> for Round {
    fn from(s: &str) -> Self {
        use Round::*;

        match s {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("Invalid string"),
        }
    }
}

fn round_score(opponent: Move, me: Move) -> u64 {
    let outcome = match (opponent, me) {
        r if Move::MAP.contains(&r) => 0,
        _ if opponent == me => 3,
        _ => 6,
    };

    me as u64 + outcome
}

fn round_score2(opponent: Move, round: Round) -> u64 {
    use Round::*;

    let outcome = match (opponent, round) {
        (mv, Lose) => mv.beats() as u64,
        (mv, Draw) => mv as u64 + 3,
        (mv, Win) => mv.loses_to() as u64 + 6,
    };

    outcome
}

fn main() {
    let lines = aoc_io::lines("input.txt").unwrap();

    let total_score: u64 = lines
        .iter()
        .map(|line| {
            let moves: Vec<_> = line.split(" ").collect();
            let opponent = moves[0].into();
            let me = moves[1].into();
            round_score(opponent, me)
        })
        .sum();

    println!("{}", total_score);

    let total_score: u64 = lines
        .iter()
        .map(|line| {
            let moves: Vec<_> = line.split(" ").collect();
            let opponent = moves[0].into();
            let round = moves[1].into();
            round_score2(opponent, round)
        })
        .sum();

    println!("{}", total_score);
}
