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
    let mut aim = 0;
    let final_position = io::stdin().lines().map(|l| match l.unwrap().split_once(" ") {
        Some((direction, amount)) => (Direction::new(direction), amount.parse::<isize>().unwrap()),
        _ => panic!()
    }).filter_map(|(direction, amount)| match direction {
        Direction::Forward => Some((amount, amount * aim)),
        Direction::Up => { aim -= amount; None },
        Direction::Down => { aim += amount; None },
    }).reduce(|acc, value| {
        (acc.0 + value.0, acc.1 + value.1)
    }).unwrap();
    println!("x={} * y={} = {}", final_position.0, final_position.1, final_position.0 * final_position.1);
}
