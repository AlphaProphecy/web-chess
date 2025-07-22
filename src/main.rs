mod board;
mod pieces;

fn main() {
    println!("Chess Game Simulation");
    let board = board::Board::new();
    let pos = (2, 1);
    if let Some(p) = board.get(pos) {
        println!("Moves {:?}", p.to_logic().get_moves(&board, pos));
    }
}
