use crate::{
    chess_piece::*,
    fen::{FenString, STARTING_FEN},
};
use macroquad::prelude::*;

pub struct ChessBoard {
    pieces: Vec<Vec<Option<ChessPiece>>>,
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

        let fen = FenString::new(STARTING_FEN.to_string());
        let board = fen.parse_fen();

        let side = (min_axis - padding) / 8.0;
        let unit_side = min_axis / 8.0;

        for i in 0..8 {
            for j in 0..8 {
                let color = if (i + j) % 2 == 0 { GRAY } else { BROWN };

                let x_pos = unit_side * j as f32;
                let y_pos = unit_side * i as f32;
                let cell = Rect {
                    x: x_pos,
                    y: y_pos,
                    w: side,
                    h: side,
                };

                draw_rectangle(cell.x, cell.y, cell.w, cell.h, color);
                match &board[i][j] {
                    Some(piece) => Self::draw_chess_piece(
                        &piece,
                        x_pos + piece.icon.height as f32 * 0.25,
                        y_pos + piece.icon.height as f32 * 0.25,
                        side * 3.0 / 4.0,
                    ),
                    None => {}
                };
            }
        }
    }

    fn draw_chess_piece(chess_piece: &ChessPiece, x: f32, y: f32, side: f32) {
        draw_texture_ex(
            Texture2D::from_image(&chess_piece.icon),
            x,
            y,
            GOLD,
            DrawTextureParams {
                dest_size: Some(Vec2::new(side, side)),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
    }

    fn has_piece(&self, row: u32, column: u32) -> Option<&ChessPiece> {
        match self.pieces.get(row as usize) {
            Some(rank) => match rank.get(column as usize) {
                Some(col) => match col {
                    Some(piece) => Some(piece),
                    None => None,
                },
                None => None,
            },
            None => None,
        }
    }
}
