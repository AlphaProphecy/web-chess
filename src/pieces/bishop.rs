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

impl PieceLogic for Bishop {
    fn get_moves(&self, board: &Board, position: Vector) -> Vec<Vector> {
        let sets = vec![(1, 1), (-1, 1), (-1, -1), (1, -1)];
        vector_movement(board, self.color, position, sets, None)
    }

    fn record_move(&mut self, _initial: Vector, _final: Vector) {}
}
