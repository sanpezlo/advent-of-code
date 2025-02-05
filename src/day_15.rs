#[derive(Copy, Clone, PartialEq, Debug)]
enum Character {
    Robot,
    Wall,
    Box,
    RightBox,
    Empty,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn evaluate(&self, position: Position) -> Position {
        match self {
            Direction::Up => (position.0, position.1 - 1),
            Direction::Down => (position.0, position.1 + 1),
            Direction::Left => (position.0 - 1, position.1),
            Direction::Right => (position.0 + 1, position.1),
        }
    }
}

type Position = (usize, usize);

struct Board {
    data: Vec<Character>,
    moves: Vec<Direction>,
    robot: Position,
    size: Position,
}

impl Board {
    fn parse(input: &str, part_b: bool) -> Board {
        let (board, moves) = input.trim().split_once("\n\n").unwrap();

        let data = if part_b {
            board
                .chars()
                .filter_map(|c| match c {
                    '#' => Some(vec![Character::Wall, Character::Wall]),
                    'O' => Some(vec![Character::Box, Character::RightBox]),
                    '@' => Some(vec![Character::Robot, Character::Empty]),
                    '.' => Some(vec![Character::Empty, Character::Empty]),
                    _ => None,
                })
                .flatten()
                .collect::<Vec<_>>()
        } else {
            board
                .chars()
                .filter_map(|c| match c {
                    '#' => Some(Character::Wall),
                    'O' => Some(Character::Box),
                    '@' => Some(Character::Robot),
                    '.' => Some(Character::Empty),
                    _ => None,
                })
                .collect::<Vec<_>>()
        };

        let size: Position = (
            board.lines().next().unwrap().len() * if part_b { 2 } else { 1 },
            board.lines().count(),
        );

        let robot = data.iter().position(|&c| c == Character::Robot).unwrap();
        let robot = (robot % size.0, robot / size.0);

        let moves = moves
            .chars()
            .filter_map(|c| match c {
                '^' => Some(Direction::Up),
                'v' => Some(Direction::Down),
                '<' => Some(Direction::Left),
                '>' => Some(Direction::Right),
                _ => None,
            })
            .collect::<Vec<_>>();

        Board {
            data,
            moves,
            size,
            robot,
        }
    }

    fn get(&self, position: Position) -> Character {
        assert!(position.0 < self.size.0 && position.1 < self.size.1);
        self.data[position.1 * self.size.0 + position.0]
    }

    fn set(&mut self, position: Position, character: Character) {
        assert!(position.0 < self.size.0 && position.1 < self.size.1);
        self.data[position.1 * self.size.0 + position.0] = character;
    }

    fn push(&mut self, position: Position, direction: Direction) -> bool {
        let character = self.get(position);

        match character {
            Character::Wall => return false,
            Character::Empty => return true,
            _ => {}
        }

        let new_position = direction.evaluate(position);

        if self.push(new_position, direction) {
            self.set(new_position, character);
            self.set(position, Character::Empty);
            return true;
        }

        false
    }

    fn can_push(&self, position: Position, direction: Direction) -> bool {
        let character = self.get(position);

        match character {
            Character::Wall => return false,
            Character::Empty => return true,
            _ => {}
        }

        match direction {
            Direction::Down | Direction::Up => {
                let next_box = match character {
                    Character::Box => Some(Direction::Right.evaluate(position)),
                    Character::RightBox => Some(Direction::Left.evaluate(position)),
                    _ => None,
                };

                if let Some(next_box) = next_box {
                    if !self.can_push(direction.evaluate(next_box), direction) {
                        return false;
                    }
                }
            }
            _ => {}
        };

        let new_position = direction.evaluate(position);

        if self.can_push(new_position, direction) {
            return true;
        }

        false
    }

    fn push_b(&mut self, position: Position, direction: Direction) -> bool {
        if !self.can_push(position, direction) {
            return false;
        }

        let character = self.get(position);

        match character {
            Character::Empty => return true,
            Character::Wall => unreachable!(),
            _ => {}
        }

        match direction {
            Direction::Up | Direction::Down => {
                let next_box = match character {
                    Character::Box => {
                        Some((Direction::Right.evaluate(position), Character::RightBox))
                    }
                    Character::RightBox => {
                        Some((Direction::Left.evaluate(position), Character::Box))
                    }
                    _ => None,
                };

                if let Some((next_box, next_character)) = next_box {
                    self.push_b(direction.evaluate(next_box), direction);
                    self.set(direction.evaluate(next_box), next_character);
                    self.set(next_box, Character::Empty);
                }
            }
            _ => {}
        }

        let new_position = direction.evaluate(position);

        if self.push_b(new_position, direction) {
            self.set(new_position, character);
            self.set(position, Character::Empty);
            return true;
        }

        false
    }

    fn tick_all(&mut self, part_b: bool) {
        for i in 0..self.moves.len() {
            let direction = self.moves[i];
            let position = self.robot;

            let can_push = if part_b {
                self.push_b(position, direction)
            } else {
                self.push(position, direction)
            };

            if can_push {
                self.robot = direction.evaluate(position);
            }
        }
    }

    fn score(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .filter(|&(_, &c)| c == Character::Box)
            .map(|(i, _)| (i / self.size.0) * 100 + (i % self.size.0))
            .sum()
    }
}

pub fn part_1(input: &str) -> usize {
    let mut board = Board::parse(input, false);
    board.tick_all(false);
    board.score()
}

pub fn part_2(input: &str) -> usize {
    let mut board = Board::parse(input, true);
    board.tick_all(true);
    board.score()
}

#[cfg(test)]
mod tests {
    const CASE: &str = "
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    ";

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(CASE), 10092);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(CASE), 9021);
    }
}
