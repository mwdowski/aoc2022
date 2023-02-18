use std::fs::read_to_string;

#[derive(Clone, Copy)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}
use Rps::*;

impl Rps {
    fn new(string: &str) -> Self {
        match string {
            "X" | "A" => Rock,
            "Y" | "B" => Paper,
            "Z" | "C" => Scissors,
            _ => unreachable!(),
        }
    }
}

enum GameResult {
    Win,
    Draw,
    Loss,
}
use GameResult::*;

impl GameResult {
    fn new(string: &str) -> Self {
        match string {
            "X" => Loss,
            "Y" => Draw,
            "Z" => Win,
            _ => unreachable!(),
        }
    }
}

fn get_game_result(other: Rps, my: Rps) -> GameResult {
    match (other, my) {
        (Rock, Scissors) => Loss,
        (Paper, Scissors) => Win,
        (Paper, Rock) => Loss,
        (Scissors, Rock) => Win,
        (Scissors, Paper) => Loss,
        (Rock, Paper) => Win,
        _ => Draw,
    }
}

fn get_target_move(other: Rps, res: GameResult) -> Rps {
    match (other, res) {
        (Rock, Loss) => Scissors,
        (Paper, Win) => Scissors,
        (Paper, Loss) => Rock,
        (Scissors, Win) => Rock,
        (Scissors, Loss) => Paper,
        (Rock, Win) => Paper,
        (my_rps, Draw) => my_rps,
    }
}

fn points_from_game(other: Rps, my: Rps) -> i32 {
    let from_my = match my {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };

    let from_result = match get_game_result(other, my) {
        Win => 6,
        Draw => 3,
        Loss => 0,
    };

    from_my + from_result
}

pub fn part1() -> i32 {
    let file_contents = read_to_string("./inputs/in02").expect("Can't open file");

    file_contents
        .split("\n")
        .map(|line| line.split(" ").map(|ch| Rps::new(ch)))
        .map(iterator_to_pair)
        .map(|(other, my)| points_from_game(other, my))
        .sum::<i32>()
}

fn iterator_to_pair<T: Copy>(iter: impl Iterator<Item = T>) -> (T, T) {
    let vec = iter.collect::<Vec<T>>();
    match &vec[..] {
        &[first, second, ..] => (first, second),
        _ => unreachable!(),
    }
}

pub fn part2() -> i32 {
    let file_contents = read_to_string("./inputs/in02").expect("Can't open file");

    file_contents
        .split("\n")
        .map(|line| line.split(" "))
        .map(iterator_to_pair)
        .map(|(abc, xyz)| {
            let rps = Rps::new(abc);
            let res = GameResult::new(xyz);

            let target_rps = get_target_move(rps, res);
            points_from_game(rps, target_rps)
        })
        .sum::<i32>()
}
