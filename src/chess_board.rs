use crate::{chess_piece::*, fen::STARTING_FEN};
use macroquad::prelude::*;

pub struct ChessBoard {
    pieces: Vec<ChessPiece>,
    moves: Vec<String>,
    fen_string: String,
}

impl ChessBoard {
    pub fn new() -> Self {
        ChessBoard {
            pieces: Vec::new(),
            moves: Vec::new(),
            fen_string: String::from(STARTING_FEN),
        }
    }
    pub fn draw_chess_board() {
        let padding = 20.0;
        let min_axis = if screen_height() < screen_width() {
            screen_height()
        } else {
            screen_width()
        };

        let side = (min_axis - padding) / 8.0;
        let unit_side = min_axis / 8.0;

        for i in 0..8 {
            for j in 0..8 {
                let color = if (i + j) % 2 == 0 { WHITE } else { BLACK };

                let x_pos = unit_side * i as f32;
                let y_pos = unit_side * j as f32;
                let cell = Rect {
                    x: x_pos,
                    y: y_pos,
                    w: side,
                    h: side,
                };
                draw_rectangle(cell.x, cell.y, cell.w, cell.h, color);
                Self::draw_chess_piece(
                    ChessPiece::new(ChessColor::Black, ChessPieceType::King),
                    cell,
                );
            }
        }
    }

    fn draw_chess_piece(chess_piece: ChessPiece, rect: Rect) {
        draw_texture(
            Texture2D::from_image(&chess_piece.icon),
            rect.x,
            rect.y,
            WHITE,
        )
    }

    fn has_piece(row: u32, column: u32) -> Option<ChessPiece> {
        unimplemented!()
    }

    fn load_from_fenstring(fen_string: String) {}
}
