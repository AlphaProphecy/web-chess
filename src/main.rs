mod board;
mod piece;

use piece::{Bishop, Color, King, Knight, Pawn, Piece, PieceLogic, Queen, Rook};

fn main() {
    let board = board::Board::new();
    let pos = (0, 3);
    if let Some(p) = board.get(pos) {
        match p {
            Piece::Rook(r) => println!("moves: {:?}", r.get_moves(&board, pos)),
            _ => println!("Not a rook"),
        }
    }
}
