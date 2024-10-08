use super::{vector_movement, Color, PieceLogic};
use crate::board::{Board, Vector};

#[derive(Debug)]
pub struct Rook {
    pub color: Color,
    pub has_moved: bool,
}

impl Rook {
    pub fn new(color: Color) -> Rook {
        Rook {
            color,
            has_moved: false,
        }
    }
}

const ROOK_SETS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

impl PieceLogic for Rook {
    fn get_moves(&self, board: &Board, position: Vector) -> Vec<Vector> {
        vector_movement(board, self.color, position, &ROOK_SETS, None)
    }

    fn record_move(&mut self, _initial: Vector, _final: Vector) {
        self.has_moved = true;
    }
}
