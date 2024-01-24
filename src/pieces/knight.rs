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

impl PieceLogic for Knight {
    fn get_moves(&self, board: &Board, position: Vector) -> Vec<Vector> {
        let sets = vec![
            (1, 2),
            (2, 1),
            (-1, 2),
            (-2, 1),
            (-1, -2),
            (-2, -1),
            (1, -2),
            (2, -1),
        ];
        vector_movement(board, self.color, position, sets, Some(1))
    }

    fn record_move(&mut self, _initial: Vector, _final: Vector) {}
}
