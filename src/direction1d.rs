use num;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction1D {
    Right,
    Left,
}

impl Direction1D {
    pub fn go<T: num::Num>(x: T, d: Direction1D) -> T {
        Direction1D::go_n(x, d, num::one())
    }

    pub fn go_n<T: num::Num>(x: T, d: Direction1D, n: T) -> T {
        match d {
            Direction1D::Right => x + n,
            Direction1D::Left => x - n,
        }
    }

    pub fn turn_around(d: Direction1D) -> Direction1D {
        match d {
            Direction1D::Right => Direction1D::Left,
            Direction1D::Left => Direction1D::Right,
        }
    }
}

pub const DIRECTIONS: [Direction1D; 2] = [Direction1D::Right, Direction1D::Left];
