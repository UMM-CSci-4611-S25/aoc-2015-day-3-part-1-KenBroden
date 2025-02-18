use std::fmt::Result;


// use std::collections::HashSet;
fn main() {
    let input_file_name = "input.txt";
    let contents = std::fs::read_to_string(input_file_name).expect("Failed to read the input file");
    println!("{}", contents.len());
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IllegalChar(char);

impl TryFrom<char> for Direction {
    type Error = IllegalChar;

    fn try_from(c:char) -> Result<Self, Self::Error> {
        Ok(match c {
            '^' => Self::North,
            'v' => Self::South,
            '<' => Self::West,
            '>' => Self::East,
            _ => return Err(IllegalChar(c)),
        })
    }
}

struct Moves{
    moves: Vec<Direction>,
}

impl FromStr for Moves {
    type Err = IllegalChar;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: Vec<Direction> = s
            .chars()
            .map(Direction::try_from)
            .collect::<Result<Vec<Direction>, IllegalChar>>()?;
        Ok (Moves { moves })
    }
}

pub struct VisitedHouses {
    visited_houses: HashSet<Pos>,
    current_position: Pos,
}

impl VisitedHouses {
    #[must_use]
    pub fn new() -> VisitedHouses {
        VisitedHouses {
            let initial_position = Pos::new(0, 0);
            let mut visited_houses: HashSet<Pos> = HashSet::new();
            visited_houses.insert(initial_position).clone();

        }
    }

    pub fn num_visited_houses(&self) -> i32 {
        1
    }

    pub fn current_pos(&self) -> Pos {
        Pos::new(0, 0)
    }

    pub fn perform_move()

    pub fn perform_moves(&mut self, moves: Moves) {
        for direction in moves.moves {
            self.perform_move(direction);
        }
    }
}

impl Default for VisitedHouses {
    fn default() -> Self {
        VisitedHouses::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visited_houses_new() {
        let /* mut */ visited_houses = VisitedHouses::new();
        assert_eq!(visited_houses.num_visited_houses(), 1);
        // assert_eq!(visited_houses.current_pos(), Pos(0, 0));
    }

    // #[test]
    // fn test_direction_try_from() {
    //     assert_eq!('^'.try_into(), Ok(Direction::North));
    //     assert_eq!('v'.try_into(), Ok(Direction::South));
    //     assert_eq!('<'.try_into(), Ok(Direction::West));
    //     assert_eq!('>'.try_into(), Ok(Direction::East));
    //     assert_eq!(Direction::try_from('x'), Err(IllegalDirectionCharacter('x')));
    // }

    // #[test]
    // fn test_move_east() {
    //     let mut visited_houses = VisitedHouses::new();
    //     visited_houses.perform_move(Direction::East);
    //     assert_eq!(visited_houses.num_visited_houses(), 2);
    //     assert_eq!(visited_houses.current_pos, Pos(1, 0));
    // }

    // #[test]
    // fn test_square_moves() {
    //     let mut visited_houses = VisitedHouses::new();
    //     let moves = Moves::from_str("^>v<").unwrap();
    //     visited_houses.perform_moves(moves);
    //     assert_eq!(visited_houses.num_visited_houses(), 4);
    //     assert_eq!(visited_houses.current_pos, Pos(0, 0));
    // }

    // #[test]
    // fn test_up_down_moves() {
    //     let mut visited_houses = VisitedHouses::new();
    //     let moves = Moves::from_str("^v^v^v^v^v").unwrap();
    //     visited_houses.perform_moves(moves);
    //     assert_eq!(visited_houses.num_visited_houses(), 2);
    //     assert_eq!(visited_houses.current_pos, Pos(0, 0));
    // }

    // #[test]
    // fn test_aoc_input() {
    //     let mut visited_houses = VisitedHouses::new();
    //     let moves = Moves::from_str(include_str!("../input.txt")).unwrap();
    //     visited_houses.perform_moves(moves);
    //     assert_eq!(visited_houses.num_visited_houses(), 2565);
    // }
}
