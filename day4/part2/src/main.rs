use std::io;

fn main() {
    // Read first line and convert it into a vector of numbers
    //
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let numbers: Vec<_> = buffer
        .trim()
        .split(',')
        .map(|str| str.parse::<usize>().unwrap())
        .collect();
    buffer.clear();

    let mut boards = vec![];

    // Read boards preceded with an empty line
    //
    while let Ok(1) = io::stdin().read_line(&mut buffer) {
        buffer.clear();
        // Read board of 5 lines each
        //
        let mut board = vec![];
        for _line in 0..5 {
            io::stdin().read_line(&mut buffer).unwrap();
            let numbers: Vec<_> = buffer
                .trim()
                .split(' ')
                .filter(|str| !str.is_empty())
                .map(|str| str.parse::<usize>().unwrap())
                .collect();
            buffer.clear();
            board.push(numbers);
        }
        //println!("{:?}", board);
        boards.push(board);
    }

    // Starting with 5 numbers check for each drawn number if any of the
    // boards has bingo.
    //
    for i in 5..=numbers.len() {
        let (winner, remaining) = boards
            .into_iter()
            .partition(|board| is_bingo(board, &numbers[0..i]));
        boards = remaining;
        println!("- - - - -\n{:?}", &numbers[0..i]);
        for board in winner {
            println!("{:?}", board);
            println!("{}", score(&board, &numbers[0..i]));
        }
    }
}

fn score(board: &[Vec<usize>], numbers: &[usize]) -> usize {
    board
        .iter()
        .map(|line| {
            line.iter()
                .filter(|number| !numbers.contains(number))
                .sum::<usize>()
        })
        .sum::<usize>()
        * numbers.last().unwrap()
}

fn is_bingo(board: &Vec<Vec<usize>>, numbers: &[usize]) -> bool {
    for line in board {
        if line.len()
            == line
                .iter()
                .map(|number| if numbers.contains(number) { 1 } else { 0 })
                .sum()
        {
            return true;
        }
    }
    for line in transposed(board) {
        if line.len()
            == line
                .iter()
                .map(|number| if numbers.contains(number) { 1 } else { 0 })
                .sum()
        {
            return true;
        }
    }
    false
}

fn transposed(board: &[Vec<usize>]) -> Vec<Vec<usize>> {
    (0..board[0].len())
        .map(|i| board.iter().map(|line| line[i]).collect())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::transposed;

    #[test]
    fn transpose_sym() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = transposed(&input);
        let expected = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
        assert_eq!(result, expected);
    }

    #[test]
    fn transpose_asym() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let result = transposed(&input);
        let expected = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        assert_eq!(result, expected);
    }
}
