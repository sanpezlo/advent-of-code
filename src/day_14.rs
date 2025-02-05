struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Robot {
    fn parse(input: &str) -> Self {
        let mut parts = input.split_whitespace();

        let pos = parts.next().unwrap();
        let vel = parts.next().unwrap();

        let parse_coord = |string: &str| {
            let (x, y) = string[2..].split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        };

        Self {
            pos: parse_coord(pos),
            vel: parse_coord(vel),
        }
    }

    fn tick(&mut self, bounds: (i32, i32), n: i32) {
        self.pos.0 = (self.pos.0 + self.vel.0 * n).rem_euclid(bounds.0);
        self.pos.1 = (self.pos.1 + self.vel.1 * n).rem_euclid(bounds.1);
    }
}

struct Board {
    robots: Vec<Robot>,
    bounds: (i32, i32),
}

impl Board {
    fn parse(input: &str) -> Board {
        let robots = input
            .lines()
            .filter(|x| !x.trim().is_empty())
            .map(Robot::parse)
            .collect::<Vec<_>>();

        let (wide, tall) = robots.iter().fold((0, 0), |acc, robot| {
            (robot.pos.0.max(acc.0), robot.pos.1.max(acc.1))
        });

        Board {
            robots,
            bounds: (wide + 1, tall + 1),
        }
    }

    fn tick(&mut self, n: i32) {
        self.robots
            .iter_mut()
            .for_each(|robot| robot.tick(self.bounds, n));
    }

    fn score(&self) -> i32 {
        let half_bounds = (self.bounds.0 / 2, self.bounds.1 / 2);

        let mut quadrants = [0; 4];
        for pos in self.robots.iter().map(|x| x.pos) {
            if pos.0 == half_bounds.0 || pos.1 == half_bounds.1 {
                continue;
            }

            let width = (0..=half_bounds.0).contains(&pos.0);
            let height = (0..=half_bounds.1).contains(&pos.1);
            quadrants[((width as usize) << 1) | height as usize] += 1;
        }

        quadrants.iter().product()
    }

    fn ticks_for_min_variances(&mut self) -> (i32, i32) {
        let max_bound = self.bounds.0.max(self.bounds.1);

        let mut min_variances = (f64::MAX, f64::MAX);
        let mut min_ticks = (0, 0);

        for i in 0..max_bound {
            let xs = self.robots.iter().map(|x| x.pos.0).collect::<Vec<_>>();
            let ys = self.robots.iter().map(|x| x.pos.1).collect::<Vec<_>>();

            let variances = (variance(&xs), variance(&ys));

            if variances.0 < min_variances.0 {
                min_variances.0 = variances.0;
                min_ticks.0 = i;
            }

            if variances.1 < min_variances.1 {
                min_variances.1 = variances.1;
                min_ticks.1 = i;
            }

            self.tick(1);
        }

        return min_ticks;
    }
}

fn variance(data: &[i32]) -> f64 {
    let average = data.iter().sum::<i32>() as f64 / data.len() as f64;
    data.iter()
        .map(|x| (*x as f64 - average).powi(2))
        .sum::<f64>()
        / (data.len() - 1) as f64
}

fn mod_inverse(x: i32, n: i32) -> i32 {
    for i in 1..n {
        if (i * x).rem_euclid(n) == 1 {
            return i;
        }
    }

    return 0;
}

pub fn part_1(input: &str) -> i32 {
    let mut board = Board::parse(input);
    board.tick(100);
    board.score()
}

pub fn part_2(input: &str) -> i32 {
    let mut board = Board::parse(input);

    let (x, y) = board.ticks_for_min_variances();

    x + (mod_inverse(board.bounds.0, board.bounds.1) * (y - x)).rem_euclid(board.bounds.1)
        * board.bounds.0
}

#[cfg(test)]
mod tests {
    const CASE: &str = "
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
    ";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(CASE), 12);
    }
}
