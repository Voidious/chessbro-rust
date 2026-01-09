use chess::{Board, ChessMove, Square};
use std::str::FromStr;

use crate::engine::Engine;

pub struct UciHandler {
    current_position: Board,
}

impl UciHandler {
    pub fn new() -> Self {
        UciHandler {
            current_position: Board::default(),
        }
    }

    pub fn handle_command(&mut self, cmd: &str, engine: &mut Engine) -> Vec<String> {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() {
            return vec![];
        }

        match parts[0] {
            "uci" => self.handle_uci(engine),
            "isready" => self.handle_isready(),
            "ucinewgame" => self.handle_ucinewgame(),
            "position" => self.handle_position(&parts[1..]),
            "go" => self.handle_go(engine),
            "quit" => vec![],
            _ => vec![], // Ignore unknown commands per UCI spec
        }
    }

    fn handle_uci(&self, engine: &Engine) -> Vec<String> {
        vec![
            format!("id name {}", engine.name),
            format!("id author {}", engine.author),
            "uciok".to_string(),
        ]
    }

    fn handle_isready(&self) -> Vec<String> {
        vec!["readyok".to_string()]
    }

    fn handle_ucinewgame(&mut self) -> Vec<String> {
        self.current_position = Board::default();
        vec![]
    }

    fn handle_position(&mut self, args: &[&str]) -> Vec<String> {
        if args.is_empty() {
            return vec![];
        }

        match self.parse_position(args) {
            Ok(board) => {
                self.current_position = board;
                vec![]
            }
            Err(e) => {
                eprintln!("Error parsing position: {}", e);
                vec![]
            }
        }
    }

    fn handle_go(&self, engine: &mut Engine) -> Vec<String> {
        match engine.get_random_move(&self.current_position) {
            Some(chess_move) => vec![format!("bestmove {}", chess_move)],
            None => vec![], // No legal moves (checkmate or stalemate)
        }
    }

    fn parse_position(&self, args: &[&str]) -> Result<Board, String> {
        let mut board = if args[0] == "startpos" {
            Board::default()
        } else if args[0] == "fen" {
            // Find where "moves" starts, or use all remaining args for FEN
            let moves_index = args.iter().position(|&x| x == "moves");
            let fen_end = moves_index.unwrap_or(args.len());

            if fen_end <= 1 {
                return Err("Invalid FEN string".to_string());
            }

            let fen_string = args[1..fen_end].join(" ");
            Board::from_str(&fen_string)
                .map_err(|_| format!("Invalid FEN string: {}", fen_string))?
        } else {
            return Err(format!("Unknown position type: {}", args[0]));
        };

        // Apply moves if present
        let moves_index = args.iter().position(|&x| x == "moves");
        if let Some(idx) = moves_index {
            for move_str in &args[idx + 1..] {
                let chess_move = parse_uci_move(&board, move_str)
                    .ok_or_else(|| format!("Invalid move: {}", move_str))?;
                board = board.make_move_new(chess_move);
            }
        }

        Ok(board)
    }
}

fn parse_uci_move(_board: &Board, move_str: &str) -> Option<ChessMove> {
    if move_str.len() < 4 {
        return None;
    }

    let from = Square::from_str(&move_str[0..2]).ok()?;
    let to = Square::from_str(&move_str[2..4]).ok()?;

    let promotion = if move_str.len() >= 5 {
        match &move_str[4..5] {
            "q" => Some(chess::Piece::Queen),
            "r" => Some(chess::Piece::Rook),
            "b" => Some(chess::Piece::Bishop),
            "n" => Some(chess::Piece::Knight),
            _ => None,
        }
    } else {
        None
    };

    Some(ChessMove::new(from, to, promotion))
}
