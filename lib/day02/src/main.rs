use std::collections::HashMap;

#[derive(PartialEq)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

fn part_one(input: String) -> u32 {
    let rps = HashMap::from([
        ("A", RockPaperScissors::Rock),
        ("B", RockPaperScissors::Paper),
        ("C", RockPaperScissors::Scissors),
        ("X", RockPaperScissors::Rock),
        ("Y", RockPaperScissors::Paper),
        ("Z", RockPaperScissors::Scissors),
    ]);
    let mut total: u32 = 0;

    for game in input.lines() {
        let (opponent, own) = game.split_at(1);
        total +=
            u32::from(get_rps_value(&rps[own.trim()])) +
            u32::from(get_result(&rps[opponent.trim()], &rps[own.trim()]));
    }
    total
}
fn get_rps_value(rps: &RockPaperScissors) -> u8 {
    let value = match Some(rps) {
        Some(RockPaperScissors::Rock) => 1,
        Some(RockPaperScissors::Paper) => 2,
        Some(RockPaperScissors::Scissors) => 3,
        None => 0,
    };
    value
}
fn get_result(opponent: &RockPaperScissors, own: &RockPaperScissors) -> u8 {
    let result: u8;
    if opponent == &RockPaperScissors::Rock {
        let v = match Some(own) {
            Some(RockPaperScissors::Rock) => 3,
            Some(RockPaperScissors::Paper) => 6,
            Some(RockPaperScissors::Scissors) => 0,
            None => 0,
        };
        result = v;
    } else if opponent == &RockPaperScissors::Paper {
        let v = match Some(own) {
            Some(RockPaperScissors::Scissors) => 6,
            Some(RockPaperScissors::Paper) => 3,
            Some(RockPaperScissors::Rock) => 0,
            None => 0,
        };
        result = v;
    } else {
        let v = match Some(own) {
            Some(RockPaperScissors::Scissors) => 3,
            Some(RockPaperScissors::Paper) => 0,
            Some(RockPaperScissors::Rock) => 6,
            None => 0,
        };
        result = v;
    }
    result
}
fn lose(rps: &RockPaperScissors) -> RockPaperScissors {
    let value = match Some(rps) {
        Some(RockPaperScissors::Rock) => RockPaperScissors::Scissors,
        Some(RockPaperScissors::Paper) => RockPaperScissors::Rock,
        Some(RockPaperScissors::Scissors) => RockPaperScissors::Paper,
        None => RockPaperScissors::Rock,
    };
    value
}

fn win(rps: &RockPaperScissors) -> RockPaperScissors {
    let value = match Some(rps) {
        Some(RockPaperScissors::Rock) => RockPaperScissors::Paper,
        Some(RockPaperScissors::Paper) => RockPaperScissors::Scissors,
        Some(RockPaperScissors::Scissors) => RockPaperScissors::Rock,
        None => RockPaperScissors::Rock,
    };
    value
}

fn part_two(input: String) -> u32 {
    let rps = HashMap::from([
        ("A", RockPaperScissors::Rock),
        ("B", RockPaperScissors::Paper),
        ("C", RockPaperScissors::Scissors),
        ("X", RockPaperScissors::Rock),
        ("Y", RockPaperScissors::Paper),
        ("Z", RockPaperScissors::Scissors),
    ]);
    let mut total: u32 = 0;

    for game in input.lines() {
        let (opponent, own) = game.split_at(1);
        if own.trim() == "Y" {
            //has to draw
            total += u32::from(get_rps_value(&rps[opponent.trim()]));
            total += 3;
        } else if own.trim() == "X" {
            //has to lose
            total += u32::from(get_rps_value(&lose(&rps[opponent.trim()])));
            total += 0;
        } else {
            // win condition
            total += u32::from(get_rps_value(&win(&rps[opponent.trim()])));
            total += 6;
        }
    }
    total
}

const INPUT: &str = "A Y
B X
C Z";

fn main() {
    println!("part one: {}", part_one(INPUT.to_string()));
    println!("part two: {}", part_two(INPUT.to_string()));
}

#[cfg(test)]
mod tests {
    use crate::part_one;
    use crate::part_two;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part_one_test() {
        assert_eq!(15, part_one(INPUT.to_string()));
    }
    #[test]
    fn part_two_test() {
        assert_eq!(12, part_two(INPUT.to_string()));
    }
}