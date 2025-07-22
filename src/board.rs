use crate::pieces::bishop::Bishop;
use crate::pieces::king::King;
use crate::pieces::knight::Knight;
use crate::pieces::pawn::Pawn;
use crate::pieces::queen::Queen;
use crate::pieces::rook::Rook;
use crate::pieces::Color;
use crate::pieces::Piece;

pub type Vector = (i32, i32);

pub const BOARD_HEIGHT: usize = 8;
pub const BOARD_WIDTH: usize = 8;
pub const IS_SQUARE: bool = BOARD_HEIGHT == BOARD_WIDTH;

pub struct Board {
    state: [[Option<Piece>; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    pub fn new() -> Board {
        Board {
            state: [
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
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
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
            ],
        }
    }

    pub fn get(&self, position: Vector) -> Option<&Piece> {
        let (x, y) = position;
        self.state[y as usize][x as usize].as_ref()
    }

    pub fn display(&self) {
        for row in self.state.iter() {
            for piece in row.iter() {
                match piece {
                    Some(piece) => print!("{} ", piece.display()),
                    None => print!(". "),
                }
            }
            println!();
        }
    }
}
