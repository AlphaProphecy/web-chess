use super::{vector_movement, Color, PieceLogic};
use crate::board::{Board, Vector};

#[derive(Debug)]
pub struct Queen {
    pub color: Color,
}

impl Queen {
    pub fn new(color: Color) -> Queen {
        Queen { color }
    }
}

const QUEEN_SETS: [(i32, i32); 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (-1, 1),
    (-1, -1),
    (1, -1),
];

impl PieceLogic for Queen {
    fn get_moves(&self, board: &Board, position: Vector) -> Vec<Vector> {
        vector_movement(board, self.color, position, &QUEEN_SETS, None)
    }

    fn record_move(&mut self, _initial: Vector, _final: Vector) {}
}
