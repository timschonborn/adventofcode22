// enum Action {
//     X, Y, Z
// }

fn actionPoints(c: char) -> usize {
    match c {
        'X' => POINTS_STONE,
        'Y' => POINTS_PAPER,
        'Z' => POINTS_SCISSORS,
        _ => panic!("Invalid action"),
    }
}

const POINTS_WIN: usize = 6;
const POINTS_LOSS: usize = 0;
const POINTS_DRAW: usize = 3;

const POINTS_STONE: usize = 1;
const POINTS_PAPER: usize = 2;
const POINTS_SCISSORS: usize = 3;

/// Play Stone-Paper-Scissors with the elves
/// 
/// A: stone
/// B: paper
/// C: scissors
/// X: stone (1)
/// Y: paper (2)
/// Z: scissors (3)
fn part1(games: core::str::Split<char>) -> usize {
    let mut points = 0;
    for game in games {
        println!("Game: {}", game);
        println!("A: {}", game.chars().nth(0).unwrap());
        // let elve = game.chars().next().unwrap();
        // let me = game.chars().nth(1).unwrap();
        // match (elve, me) {
        //     ('A', 'X') | ('B', 'Y') | ('C', 'Z') => {
        //         points += POINTS_DRAW;
        //         points += actionPoints(me)
        //     },
        //     ('A', 'Y') | ('B', 'Z') | ('C', 'X') => {
        //         points += POINTS_WIN;
        //         points += actionPoints(me)
        //     },
        //     ('A', 'Z') | ('B', 'X') | ('C', 'Y') => {
        //         points += POINTS_LOSS;
        //         points += actionPoints(me)
        //     },
        //     _ => println!("Invalid input"),
        // }
    }
    points
}

fn main() {
    let s = include_str!("input.txt").split('\n');

    println!("Part 1: {}", part1(s));
}
