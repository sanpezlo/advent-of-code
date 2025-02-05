pub fn part_1(input: &str) -> i32 {
    let reports = parse(input);
    reports.iter().filter(|&x| is_safe(x)).count() as i32
}

pub fn part_2(input: &str) -> i32 {
    let reports = parse(input);
    let a = reports.iter().filter(|&x| is_safe_2(x, None));

    a.count() as i32
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|x| x.len() > 0)
        .collect::<Vec<_>>()
}

fn is_safe(input: &[i32]) -> bool {
    let sig = (input[0] - input[1]).signum();

    input
        .iter()
        .take(input.len() - 1)
        .zip(input.iter().skip(1))
        .map(|(a, b)| a - b)
        .all(|x| (1..=3).contains(&x.abs()) && x.signum() == sig)
}

fn is_safe_2(input: &[i32], skip_index: Option<usize>) -> bool {
    let data = input
        .iter()
        .enumerate()
        .filter(|(i, _)| Some(*i) != skip_index)
        .map(|(_, &x)| x)
        .collect::<Vec<_>>();

    let mut iter = data
        .iter()
        .take(data.len() - 1)
        .zip(data.iter().skip(1))
        .map(|(a, b)| a - b)
        .peekable();

    let sig = iter.peek().unwrap().signum();
    let invalid = iter.position(|x| !(1..=3).contains(&x.abs()) || x.signum() != sig);

    if let Some(x) = invalid {
        if skip_index.is_some() {
            return false;
        }

        return is_safe_2(input, Some(x + 1))
            || is_safe_2(input, Some(x.saturating_sub(1)))
            || is_safe_2(input, Some(x));
    }

    true
}

#[cfg(test)]
mod tests {
    const CASE: &str = "
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    ";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(CASE), 2);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(CASE), 4);
    }
}
