use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::UP => Direction::Down,
            Direction::Down => Direction::UP,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
