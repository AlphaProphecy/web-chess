use crate::board::{Board, Vector};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

// impl Color {
//     pub fn direction(&self) -> i32 {
//         match self {
//             Color::White => 1,
//             Color::Black => -1,
//         }
//     }
// }

pub trait PieceLogic {
    fn get_moves(&self, board: &Board, position: Vector) -> Vec<Vector>;
    fn record_move(&mut self, inital: Vector, final_: Vector);
}

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

    fn record_move(&mut self, _inital: Vector, _final_: Vector) {}
}

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

    fn record_move(&mut self, _inital: Vector, _final_: Vector) {}
}

pub struct Rook {
    pub color: Color,
}

impl Rook {
    pub fn new(color: Color) -> Rook {
        Rook { color }
    }
}

impl PieceLogic for Rook {
    fn get_moves(&self, board: &Board, position: Vector) -> Vec<Vector> {
        let sets = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        vector_movement(board, self.color, position, sets, None)
    }

    fn record_move(&mut self, _inital: Vector, _final_: Vector) {}
}

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

    fn record_move(&mut self, _inital: Vector, _final_: Vector) {}
}

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
        vector_movement(board, self.color, position, sets, Some(1))
    }

    fn record_move(&mut self, _inital: Vector, _final_: Vector) {
        self.has_moved = true;
    }
}

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
        let sets = vec![(0, 1)];
        let limit = match self.has_moved {
            true => Some(1),
            false => Some(2),
        };
        let mut moves = vector_movement(board, self.color, position, sets, limit);
        moves
    }

    fn record_move(&mut self, _inital: Vector, _final_: Vector) {
        self.has_moved = true;
        if (_inital.1 - _final_.1).abs() == 2 {
            self.did_double_move = true;
        }
    }
}

pub enum Piece {
    Knight(Knight),
    Bishop(Bishop),
    Rook(Rook),
    Queen(Queen),
    King(King),
    Pawn(Pawn),
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
}

fn vector_movement(
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
