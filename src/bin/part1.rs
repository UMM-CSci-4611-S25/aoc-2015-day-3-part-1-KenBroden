use std::{collections::HashSet, hash::Hash, ops::Add, str::FromStr};

fn main() {
    let input_file_name = "input.txt";
    let contents = std::fs::read_to_string(input_file_name).expect("Failed to read the input file");
    println!("{}", contents.len());
}

// Framework of a Position struct
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Pos {
    x: i32,
    y: i32,
}

// Creates a new instance of the Pos struct
impl Pos {
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

// This allows us to add a direction to a position
impl Add<Direction> for Pos {
    type Output = Self;

    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::North => Self::new(self.x, self.y + 1),
            Direction::South => Self::new(self.x, self.y - 1),
            Direction::East => Self::new(self.x + 1, self.y),
            Direction::West => Self::new(self.x - 1, self.y),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct IllegalChar(char);

// Actions for the possible input characters
impl TryFrom<char> for Direction {
    type Error = IllegalChar;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '^' => Self::North,
            'v' => Self::South,
            '<' => Self::West,
            '>' => Self::East,
            _ => return Err(IllegalChar(c)),
        })
    }
}

// Creates the framework for the Moves struct
pub struct Moves {
    moves: Vec<Direction>,
}

// Defines the actions for the Moves struct
impl FromStr for Moves {
    type Err = IllegalChar;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s
            .chars()
            .map(Direction::try_from)
            .collect::<Result<Vec<Direction>, IllegalChar>>()?;
        Ok(Moves { moves })
    }
}

// Creates the framework for the VisitedHouses struct
pub struct VisitedHouses {
    visited_houses: HashSet<Pos>,
    current_position: Pos,
}

// Implementations for the VisitedHouses struct
impl VisitedHouses {
    // Creates a new instance of the VisitedHouses struct
    #[must_use]
    pub fn new() -> Self {
        let initial_position = Pos::new(0, 0);
        let mut visited_houses = HashSet::new();
        visited_houses.insert(initial_position);

        Self {
            visited_houses,
            current_position: initial_position,
        }
    }

    // Keeps a count of the number of houses visited
    #[must_use]
    pub fn num_visited_houses(&self) -> usize {
        self.visited_houses.len()
    }

    // Keeps track of the current position
    #[must_use]
    pub fn current_pos(&self) -> Pos {
        self.current_position
    }

    // Updates the current position and adds the new position to the visited houses
    pub fn perform_move(&mut self, direction: Direction) {
        let new_position = self.current_position + direction;
        self.current_position = new_position;
        self.visited_houses.insert(new_position);
    }

    // Performs a series of moves
    pub fn perform_moves(&mut self, moves: Moves) {
        for m in moves.moves {
            self.perform_move(m);
        }
    }
}

// The base implementation for the VisitedHouses struct
impl Default for VisitedHouses {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visited_houses_new() {
        let visited_houses = VisitedHouses::new();
        assert_eq!(visited_houses.num_visited_houses(), 1);
        assert_eq!(visited_houses.current_pos(), Pos::new(0, 0));
    }

    #[test]
    fn test_direction_try_from() {
        assert_eq!('^'.try_into(), Ok(Direction::North));
        assert_eq!('v'.try_into(), Ok(Direction::South));
        assert_eq!('<'.try_into(), Ok(Direction::West));
        assert_eq!('>'.try_into(), Ok(Direction::East));
        assert_eq!(Direction::try_from('x'), Err(IllegalChar('x')));
    }

    #[test]
    fn test_move_east() {
        let mut visited_houses = VisitedHouses::new();
        visited_houses.perform_move(Direction::East);
        assert_eq!(visited_houses.num_visited_houses(), 2);
        assert_eq!(visited_houses.current_pos(), Pos::new(1, 0));
    }

    #[test]
    fn test_square_moves() {
        let mut visited_houses = VisitedHouses::new();
        let moves = Moves::from_str("^>v<").unwrap();
        visited_houses.perform_moves(moves);
        assert_eq!(visited_houses.num_visited_houses(), 4);
        assert_eq!(visited_houses.current_pos(), Pos::new(0, 0));
    }

    #[test]
    fn test_up_down_moves() {
        let mut visited_houses = VisitedHouses::new();
        let moves = Moves::from_str("^v^v^v^v^v").unwrap();
        visited_houses.perform_moves(moves);
        assert_eq!(visited_houses.num_visited_houses(), 2);
        assert_eq!(visited_houses.current_pos(), Pos::new(0, 0));
    }

    #[test]
    fn test_aoc_input() {
        let mut visited_houses = VisitedHouses::new();
        let moves = Moves::from_str(include_str!("../../input.txt").trim()).unwrap();
        visited_houses.perform_moves(moves);
        assert_eq!(visited_houses.num_visited_houses(), 2565);
    }
}
