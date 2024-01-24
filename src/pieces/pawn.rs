use super::{vector_movement, Color, PieceLogic};
use crate::board::{Board, Vector};

#[derive(Debug)]
pub struct Pawn {
    pub color: Color,
    pub has_moved: bool,
    pub did_double_move: bool,
}

impl Pawn {
    pub fn new(color: Color) -> Pawn {
        Pawn {
            color,
            has_moved: false,
            did_double_move: false,
        }
    }
}

impl PieceLogic for Pawn {
    fn get_moves(&self, board: &Board, position: Vector) -> Vec<Vector> {
        let dir = self.color.direction();
        let sets = vec![(0, dir)];
        let limit = match self.has_moved {
            true => Some(1),
            false => Some(2),
        };
        let mut moves = vector_movement(board, self.color, position, sets, limit);

        let y = position.1 + dir;
        if y < 0 || y > 7 {
            return moves;
        }

        let sets = [(1, dir), (-1, dir)];
        for (x, y) in sets {
            let (x, y) = (position.0 + x, position.1 + y);
            if let Some(piece) = board.get((x, y)) {
                if piece.color() != self.color {
                    moves.push((x, y));
                }
            }
        }

        moves
    }

    fn record_move(&mut self, _initial: Vector, _final: Vector) {
        self.has_moved = true;
        if (_initial.1 - _final.1).abs() == 2 {
            self.did_double_move = true;
        }
    }
}
