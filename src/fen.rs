use crate::chess_piece::{ChessColor, ChessPiece, ChessPieceType};

pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

struct FenString {
    fen_string: String,
}

impl FenString {
    pub fn new(value: String) -> Self {
        FenString { fen_string: value }
    }

    pub fn match_piece_type(notation: char) -> Option<ChessPiece> {
        if notation.is_digit(10) {
            return None;
        }

        let piece_type = match notation.to_ascii_lowercase() {
            'r' => ChessPieceType::Rook,
            'n' => ChessPieceType::Knight,
            'b' => ChessPieceType::Bishop,
            'k' => ChessPieceType::King,
            'q' => ChessPieceType::Queen,
            'p' => ChessPieceType::Pawn,
            _ => return None,
        };

        let color = if notation.is_lowercase() {
            ChessColor::Black
        } else {
            ChessColor::White
        };

        Some(ChessPiece::new(color, piece_type))
    }

    fn parse_fen(&self) -> Vec<Vec<Option<ChessPiece>>> {
        let mut pieces_matrix: Vec<Vec<Option<ChessPiece>>> = Vec::new();
        let split_fen = self.fen_string.split("/");

        // Just to make sure the fen is size 8
        assert_eq!(split_fen.size_hint().0, 8);

        for (i, item) in split_fen.enumerate() {
            let mut col_counter = 0;
            for (j, notation) in item.chars().enumerate() {
                match Self::match_piece_type(notation) {
                    Some(piece) => pieces_matrix[i][j + col_counter] = Some(piece),
                    None => {
                        assert!(notation.is_digit(10));
                        let empties = notation.to_digit(10).unwrap();
                        for k in 0..empties {
                            pieces_matrix[i][j + k as usize] = None;
                            col_counter += 1
                        }
                    }
                }
            }
        }

        assert_eq!(pieces_matrix.len(), 0);

        pieces_matrix
    }
}

mod tests {
    use crate::{
        chess_piece::{ChessColor, ChessPieceType},
        fen::FenString,
    };

    use super::STARTING_FEN;

    #[test]
    fn instantiating_black_king_returns_correct_piece() {
        // Should return black king
        let black_king = FenString::match_piece_type('k');
        assert!(black_king.is_some());
        let black_king = black_king.unwrap();
        assert_eq!(black_king.piece_type, ChessPieceType::King);
        assert_eq!(black_king.color, ChessColor::Black)
    }

    #[test]
    fn instantiating_white_pawn_returns_correct_piece() {
        // Should return white pawn
        let white_pawn = FenString::match_piece_type('P');
        assert!(white_pawn.is_some());
        let white_pawn = white_pawn.unwrap();
        assert_eq!(white_pawn.piece_type, ChessPieceType::Pawn);
        assert_eq!(white_pawn.color, ChessColor::White)
    }

    #[test]
    fn parse_fen_is_correct() {
        let fen = FenString::new(STARTING_FEN.to_string());
        let board = fen.parse_fen();
        // Make sure first its an 8x8 matrix
        assert_eq!(board.len(), 8);
    }
}
