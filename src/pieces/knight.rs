use super::{vector_movement, Color, PieceLogic};
use crate::board::{Board, Vector};

#[derive(Debug)]
pub struct Knight {
    pub color: Color,
}

impl Knight {
    pub fn new(color: Color) -> Knight {
        Knight { color }
    }
}

const KNIGHT_SETS: [(i32, i32); 8] = [
    (1, 2),
    (2, 1),
    (-1, 2),
    (-2, 1),
    (-1, -2),
    (-2, -1),
    (1, -2),
    (2, -1),
];

impl PieceLogic for Knight {
    fn get_moves(&self, board: &Board, position: Vector) -> Vec<Vector> {
        vector_movement(board, self.color, position, &KNIGHT_SETS, Some(1))
    }

    fn record_move(&mut self, _initial: Vector, _final: Vector) {}
}
