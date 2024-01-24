mod board;
mod pieces;

fn main() {
    let board = board::Board::new();
    let pos = (4, 3);
    if let Some(p) = board.get(pos) {
        println!("Moves {:?}", p.to_logic().get_moves(&board, pos));
    }
}
