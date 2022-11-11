use std::io;

#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down,
}

impl Direction {
    fn new(string: &str) -> Self {
        if string.eq("forward") {
            Self::Forward
        } else if string.eq("up") {
            Self::Up
        } else if string.eq("down") {
            Self::Down
        } else {
            panic!()
        }
    }
}
fn main() {
    let final_position = io::stdin().lines().map(|l| match l.unwrap().split_once(" ") {
        Some((direction, amount)) => (Direction::new(direction), amount.parse::<isize>().unwrap()),
        _ => panic!()
    }).map(|(direction, amount)| match direction {
        Direction::Forward => (amount, 0),
        Direction::Up => (0, -amount),
        Direction::Down => (0, amount),
    }).reduce(|acc, value| {
        (acc.0 + value.0, acc.1 + value.1)
    }).unwrap();
    println!("x={} * y={} = {}", final_position.0, final_position.1, final_position.0 * final_position.1);
}
