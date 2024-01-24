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

impl PieceLogic for Queen {
    fn get_moves(&self, board: &Board, position: Vector) -> Vec<Vector> {
        let sets = vec![
            (1, 0),
            (0, 1),
            (-1, 0),
            (0, -1),
            (1, 1),
            (-1, 1),
            (-1, -1),
            (1, -1),
        ];
        vector_movement(board, self.color, position, sets, None)
    }

    fn record_move(&mut self, _initial: Vector, _final: Vector) {}
}
