use std::io;

fn values_from_stdin() -> Vec<u32> {
    io::stdin()
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect()
}

fn average_over_3(values: Vec<u32>) -> Vec<u32> {
    values
        .iter()
        .skip(2)
        .zip(values.iter().skip(1))
        .map(|(next, prev)| next + prev)
        .zip(values.iter())
        .map(|(next, prev)| next + prev)
        .collect()
}

fn main() {
    let raw_values = values_from_stdin();

    let values = average_over_3(raw_values);

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
