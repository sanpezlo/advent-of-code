pub fn part_1(input: &str) -> u32 {
    let (mut list_1, mut list_2) = parse(input);

    list_1.sort();
    list_2.sort();

    list_1.iter().zip(list_2).map(|(a, b)| a.abs_diff(b)).sum()
}

pub fn part_2(input: &str) -> u32 {
    let (list_1, list_2) = parse(input);

    list_1
        .iter()
        .map(|a| {
            let count = list_2.iter().filter(|&&b| a % b == 0).count();
            a * count as u32
        })
        .sum()
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (mut list_1, mut list_2) = (Vec::new(), Vec::new());

    for str in input.lines() {
        let mut parts = str.split_whitespace();

        if let (Some(num_1), Some(num_2)) = (parts.next(), parts.next()) {
            list_1.push(num_1.parse().unwrap());
            list_2.push(num_2.parse().unwrap());
        }
    }

    (list_1, list_2)
}

#[cfg(test)]
mod tests {
    const CASE: &str = "
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    ";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(CASE), 11);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(CASE), 31);
    }
}
