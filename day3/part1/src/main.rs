use std::io;

/// A Value is represented in binary format as a collection
/// of 1s and 0s
type Value = Vec<usize>;
type Values = Vec<Value>;

fn main() {
    let values = values_from_stdin();

    let ones = vertical_sum(&values);
    let zeros = vertical_sum(&inverted(&values));

    let gamma = derive_gamma(&zeros, &ones);
    let epsilon = inverted_value(&gamma);

    println!(
        "{} * {} = {}",
        as_int(&gamma),
        as_int(&epsilon),
        as_int(&gamma) * as_int(&epsilon)
    );
}


fn values_from_stdin() -> Values {
    io::stdin()
        .lines()
        .map(|l|
            l.unwrap()
                .into_bytes()
                .iter()
                .map(|b| (b - 0x30) as usize)
                .collect::<Vec<_>>()
        )
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


fn derive_gamma(zeros: &Value, ones: &Value) -> Value {
    zeros
        .iter()
        .zip(ones.iter())
        .map(|(zeros, ones)| if zeros > ones { 0 } else { 1 })
        .collect::<Vec<_>>()
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
