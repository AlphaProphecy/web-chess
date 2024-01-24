use super::{vector_movement, Color, Piece, PieceLogic};
use crate::board::{Board, Vector};
use std::cmp::{max, min};

#[derive(Debug)]
pub struct King {
    pub color: Color,
    pub has_moved: bool,
}

impl King {
    pub fn new(color: Color) -> King {
        King {
            color,
            has_moved: false,
        }
    }
}

impl PieceLogic for King {
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
        let mut moves = vector_movement(board, self.color, position, sets, Some(1));

        if self.has_moved {
            return moves;
        }

        let starts = [(0, position.1), (7, position.1)];
        for (x, y) in starts.iter() {
            let rook = match board.get((*x, *y)) {
                Some(Piece::Rook(rook)) => rook,
                _ => continue,
            };

            if rook.color != self.color || rook.has_moved {
                continue;
            }

            let mut blocked = false;
            let start = min(*x, position.0) + 1;
            let end = max(*x, position.0);
            for x in start..end {
                if board.get((x, position.1)).is_some() {
                    blocked = true;
                    break;
                }
                // TODO: Check for check
            }
            if !blocked {
                moves.push((position.0 + (2 * ((*x - position.0).signum())), position.1));
            }
        }

        moves
    }

    fn record_move(&mut self, _initial: Vector, _final: Vector) {
        self.has_moved = true;
    }
}
