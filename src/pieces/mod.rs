use crate::board::{Board, Vector};

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn direction(&self) -> i32 {
        match self {
            Color::White => 1,
            Color::Black => -1,
        }
    }
}

pub trait PieceLogic {
    fn get_moves(&self, board: &Board, position: Vector) -> Vec<Vector>;
    fn record_move(&mut self, initial: Vector, final_: Vector);
}

#[derive(Debug)]
pub enum Piece {
    Knight(knight::Knight),
    Bishop(bishop::Bishop),
    Rook(rook::Rook),
    Queen(queen::Queen),
    King(king::King),
    Pawn(pawn::Pawn),
}

impl Piece {
    // pub fn display(&self) -> String {
    //     match self {
    //         Piece::Knight(_) => "N".to_string(),
    //         Piece::Bishop(_) => "B".to_string(),
    //         Piece::Rook(_) => "R".to_string(),
    //         Piece::Queen(_) => "Q".to_string(),
    //         Piece::King(_) => "K".to_string(),
    //         Piece::Pawn(_) => "P".to_string(),
    //     }
    // }

    pub fn color(&self) -> Color {
        match self {
            Piece::Knight(knight) => knight.color,
            Piece::Bishop(bishop) => bishop.color,
            Piece::Rook(rook) => rook.color,
            Piece::Queen(queen) => queen.color,
            Piece::King(king) => king.color,
            Piece::Pawn(pawn) => pawn.color,
        }
    }

    pub fn is_king(&self) -> bool {
        match self {
            Piece::King(_) => true,
            _ => false,
        }
    }

    pub fn to_logic(&self) -> &dyn PieceLogic {
        match self {
            Piece::Knight(knight) => knight,
            Piece::Bishop(bishop) => bishop,
            Piece::Rook(rook) => rook,
            Piece::Queen(queen) => queen,
            Piece::King(king) => king,
            Piece::Pawn(pawn) => pawn,
        }
    }
}

pub fn vector_movement(
    board: &Board,
    color: Color,
    position: Vector,
    directions: Vec<Vector>,
    limit: Option<i32>,
) -> Vec<Vector> {
    let limit = limit.unwrap_or(7);
    let mut moves = Vec::new();
    for direction in directions {
        let mut pos = position;
        for _ in 0..limit {
            pos = (pos.0 + direction.0, pos.1 + direction.1);
            let (x, y) = pos;
            if x < 0 || x > 7 || y < 0 || y > 7 {
                break;
            }
            if let Some(piece) = board.get((x, y)) {
                if piece.color() == color || piece.is_king() {
                    break;
                }
                moves.push((x, y));
                break;
            }
            moves.push((x, y));
        }
    }
    moves
}
