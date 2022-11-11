use std::io;

/// A Value is represented in binary format as a collection
/// of 1s and 0s
type Value = Vec<usize>;
type Values = Vec<Value>;

fn main() {
    let values = values_from_stdin();

    let oxygen_generator_rating = bitwise_filtering(&values, &filter_by_nth_most_common_bit)
        .first()
        .unwrap()
        .clone();

    println!(
        "oxygen generator rating = {} {:?}",
        as_int(&oxygen_generator_rating),
        oxygen_generator_rating
    );

    let co2_scrubber_rating = bitwise_filtering(&values, &filter_by_nth_least_common_bit)
        .first()
        .unwrap()
        .clone();

    println!(
        "CO2 scrubber rating = {} {:?}",
        as_int(&co2_scrubber_rating),
        co2_scrubber_rating
    );

    println!(
        "life support rating = {}",
        as_int(&oxygen_generator_rating) * as_int(&co2_scrubber_rating)
    )
}

fn bitwise_filtering(values: &Values, filter: &dyn Fn(Values, usize) -> Values) -> Vec<Value> {
    let mut filtered_values = values.clone();
    for i in 0..values[0].len() {
        if filtered_values.len() == 1 {
            break;
        }
        filtered_values = filter(filtered_values, i);
    }
    filtered_values
}

fn filter_by_nth_most_common_bit(values: Values, bit_pos: usize) -> Values {
    let most_common_bit = most_common_bits(&count_zeros(&values), &count_ones(&values))[bit_pos];
    values
        .into_iter()
        .filter(|value| value[bit_pos] == most_common_bit)
        .collect::<Values>()
}

fn filter_by_nth_least_common_bit(values: Values, bit_pos: usize) -> Values {
    let least_common_bit = least_common_bits(&count_zeros(&values), &count_ones(&values))[bit_pos];
    values
        .into_iter()
        .filter(|value| value[bit_pos] == least_common_bit)
        .collect::<Values>()
}

fn count_zeros(values: &Values) -> Value {
    vertical_sum(&inverted(&values))
}

fn count_ones(values: &Values) -> Value {
    vertical_sum(&values)
}

fn values_from_stdin() -> Values {
    io::stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .into_bytes()
                .iter()
                .map(|b| (b - 0x30) as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn vertical_sum(values: &Values) -> Value {
    values
        .clone()
        .into_iter()
        .reduce(|acc, item| {
            acc.iter()
                .zip(item.iter())
                .map(|(a, i)| a + i)
                .collect::<Vec<_>>()
                .to_vec()
        })
        .unwrap()
        .to_vec()
}

fn inverted(values: &Values) -> Values {
    values.iter().map(|value| inverted_value(value)).collect()
}

fn inverted_value(value: &Value) -> Value {
    value
        .iter()
        .map(|value| match value {
            0 => 1,
            1 => 0,
            _ => panic!(),
        })
        .collect()
}

fn most_common_bits(zeros: &Value, ones: &Value) -> Value {
    zeros
        .iter()
        .zip(ones.iter())
        .map(|(zeros, ones)| if zeros > ones { 0 } else { 1 })
        .collect::<Vec<_>>()
}

fn least_common_bits(zeros: &Value, ones: &Value) -> Value {
    inverted_value(&most_common_bits(zeros, ones))
}

#[cfg(test)]
mod test {
    use crate::as_int;

    #[test]
    fn single_bit() {
        assert_eq!(0, as_int(&vec![0]));
        assert_eq!(1, as_int(&vec![1]));
    }

    #[test]
    fn two_bits() {
        assert_eq!(0, as_int(&vec![0, 0]));
        assert_eq!(1, as_int(&vec![0, 1]));
        assert_eq!(2, as_int(&vec![1, 0]));
        assert_eq!(3, as_int(&vec![1, 1]));
    }

    #[test]
    fn more_bits() {
        assert_eq!(9, as_int(&vec![0, 1, 0, 0, 1]));
        assert_eq!(22, as_int(&vec![1, 0, 1, 1, 0]));
    }
}

fn as_int(value: &Value) -> usize {
    value.clone().into_iter().reduce(|a, v| a * 2 + v).unwrap()
}
