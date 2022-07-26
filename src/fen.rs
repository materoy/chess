use crate::chess_piece::{ChessColor, ChessPiece, ChessPieceType};

pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

struct FenString {
    fen_string: String,
}

impl FenString {
    pub fn match_piece_type(notation: char) -> Option<ChessPiece> {
        let piece_type = match notation {
            'r' => ChessPieceType::Rook,
            'n' => ChessPieceType::Knight,
            _ => return None,
        };

        let color = if notation.is_lowercase() {
            ChessColor::Black
        } else {
            ChessColor::White
        };

        Some(ChessPiece {
            color,
            piece_type,
            icon: todo!(),
        })
    }

    fn parse_fen(&self) {
        let split_fen = self.fen_string.split("/");
        for i in split_fen {
            for piece_notation in i.chars() {
                let chess_piece = Self::match_piece_type(piece_notation);
            }
        }
    }
}
