use chess::{Board, ChessMove, MoveGen};
use rand::Rng;

pub struct Engine {
    pub name: String,
    pub author: String,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            name: "ChessBro".to_string(),
            author: "ChessBro Team".to_string(),
        }
    }

    pub fn get_random_move(&self, board: &Board) -> Option<ChessMove> {
        let legal_moves: Vec<ChessMove> = MoveGen::new_legal(board).collect();

        if legal_moves.is_empty() {
            return None;
        }

        let index = rand::thread_rng().gen_range(0..legal_moves.len());
        Some(legal_moves[index])
    }
}
