#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction2D {
    Up,
    Right,
    Down,
    Left,
}

impl Direction2D {
    pub fn go<T: num::Num>((x, y): (T, T), d: Direction2D) -> (T, T) {
        Direction2D::go_n((x, y), d, num::one())
    }

    pub fn go_n<T: num::Num>((x, y): (T, T), d: Direction2D, n: T) -> (T, T) {
        match d {
            Direction2D::Up => (x, y - n),
            Direction2D::Right => (x + n, y),
            Direction2D::Down => (x, y + n),
            Direction2D::Left => (x - n, y),
        }
    }

    pub fn turn_right(d: Direction2D) -> Direction2D {
        match d {
            Direction2D::Up => Direction2D::Right,
            Direction2D::Right => Direction2D::Down,
            Direction2D::Down => Direction2D::Left,
            Direction2D::Left => Direction2D::Up,
        }
    }

    pub fn turn_left(d: Direction2D) -> Direction2D {
        match d {
            Direction2D::Up => Direction2D::Left,
            Direction2D::Right => Direction2D::Up,
            Direction2D::Down => Direction2D::Right,
            Direction2D::Left => Direction2D::Down,
        }
    }

    pub fn turn_around(d: Direction2D) -> Direction2D {
        match d {
            Direction2D::Up => Direction2D::Down,
            Direction2D::Right => Direction2D::Left,
            Direction2D::Down => Direction2D::Up,
            Direction2D::Left => Direction2D::Right,
        }
    }
}

pub const DIRECTIONS: [Direction2D; 4] = [
    Direction2D::Up,
    Direction2D::Right,
    Direction2D::Down,
    Direction2D::Left,
];
