use super::{vector_movement, Color, PieceLogic};
use crate::board::{Board, Vector};

#[derive(Debug)]
pub struct Bishop {
    pub color: Color,
}

impl Bishop {
    pub fn new(color: Color) -> Bishop {
        Bishop { color }
    }
}

const BISHOP_SETS: [(i32, i32); 4] = [(1, 1), (-1, 1), (-1, -1), (1, -1)];

impl PieceLogic for Bishop {
    fn get_moves(&self, board: &Board, position: Vector) -> Vec<Vector> {
        vector_movement(board, self.color, position, &BISHOP_SETS, None)
    }

    fn record_move(&mut self, _initial: Vector, _final: Vector) {}
}
