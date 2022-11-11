use std::io;

fn values_from_stdin() -> Vec<u32> {
    io::stdin()
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect()
}

fn main() {
    let values = values_from_stdin();

    let next_values = values.iter().skip(1);

    let increases =
        next_values
            .zip(values.iter())
            .filter_map(|(next, prev)| if next > prev { Some(true) } else { None });

    println!(
        "{} measurements are larger than the previous measurement",
        increases.count()
    );
}
