use crate::piece::Bishop;
use crate::piece::Color;
use crate::piece::King;
use crate::piece::Knight;
use crate::piece::Pawn;
use crate::piece::Piece;
use crate::piece::Queen;
use crate::piece::Rook;

pub type Vector = (i32, i32);

pub struct Board {
    state: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn new() -> Board {
        let fill_empty = |_| None;
        Board {
            state: [
                [
                    Some(Piece::Rook(Rook::new(Color::Black))),
                    Some(Piece::Knight(Knight::new(Color::Black))),
                    Some(Piece::Bishop(Bishop::new(Color::Black))),
                    Some(Piece::Queen(Queen::new(Color::Black))),
                    Some(Piece::King(King::new(Color::Black))),
                    Some(Piece::Bishop(Bishop::new(Color::Black))),
                    Some(Piece::Knight(Knight::new(Color::Black))),
                    Some(Piece::Rook(Rook::new(Color::Black))),
                ],
                [
                    Some(Piece::Pawn(Pawn::new(Color::Black))),
                    Some(Piece::Pawn(Pawn::new(Color::Black))),
                    Some(Piece::Pawn(Pawn::new(Color::Black))),
                    Some(Piece::Pawn(Pawn::new(Color::Black))),
                    Some(Piece::Pawn(Pawn::new(Color::Black))),
                    Some(Piece::Pawn(Pawn::new(Color::Black))),
                    Some(Piece::Pawn(Pawn::new(Color::Black))),
                    Some(Piece::Pawn(Pawn::new(Color::Black))),
                ],
                [(); 8].map(fill_empty),
                [
                    Some(Piece::Rook(Rook::new(Color::Black))),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
                [(); 8].map(fill_empty),
                [(); 8].map(fill_empty),
                [
                    Some(Piece::Pawn(Pawn::new(Color::White))),
                    Some(Piece::Pawn(Pawn::new(Color::White))),
                    Some(Piece::Pawn(Pawn::new(Color::White))),
                    Some(Piece::Pawn(Pawn::new(Color::White))),
                    Some(Piece::Pawn(Pawn::new(Color::White))),
                    Some(Piece::Pawn(Pawn::new(Color::White))),
                    Some(Piece::Pawn(Pawn::new(Color::White))),
                    Some(Piece::Pawn(Pawn::new(Color::White))),
                ],
                [
                    Some(Piece::Rook(Rook::new(Color::White))),
                    Some(Piece::Knight(Knight::new(Color::White))),
                    Some(Piece::Bishop(Bishop::new(Color::White))),
                    Some(Piece::Queen(Queen::new(Color::White))),
                    Some(Piece::King(King::new(Color::White))),
                    Some(Piece::Bishop(Bishop::new(Color::White))),
                    Some(Piece::Knight(Knight::new(Color::White))),
                    Some(Piece::Rook(Rook::new(Color::White))),
                ],
            ],
        }
    }

    pub fn get(&self, position: Vector) -> Option<&Piece> {
        let (x, y) = position;
        self.state[y as usize][x as usize].as_ref()
    }

    // pub fn display(&self) {
    //     for row in self.state.iter() {
    //         for piece in row.iter() {
    //             match piece {
    //                 Some(piece) => print!("{} ", piece.display()),
    //                 None => print!(". "),
    //             }
    //         }
    //         println!();
    //     }
    // }
}
