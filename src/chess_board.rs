use crate::{chess_piece::*, fen::FenString};
use macroquad::prelude::*;

pub struct ChessBoard {
    pieces: Vec<Vec<Option<ChessPiece>>>,
    moves: Vec<String>,
    fen_string: FenString,
    selected_cell: Option<(f32, f32)>,
    selected_piece_index: Option<(usize, usize)>,
}

impl ChessBoard {
    pub fn new() -> Self {
        ChessBoard {
            pieces: Vec::new(),
            moves: Vec::new(),
            fen_string: FenString::new_game(),
            selected_cell: None,
            selected_piece_index: None,
        }
    }

    pub fn update_fen_string(&mut self, new_fen: String) {
        self.fen_string = FenString::new(new_fen);
    }

    pub fn draw_chess_board(&mut self) {
        let padding = 20.0;
        let min_axis = if screen_height() < screen_width() {
            screen_height()
        } else {
            screen_width()
        };

        let mut board = self.fen_string.parse_fen();

        let side = (min_axis - padding) / 8.0;
        let unit_side = min_axis / 8.0;

        for i in 0..8 {
            for j in 0..8 {
                let mut color = if (i + j) % 2 == 0 { GRAY } else { BROWN };

                let x_pos = unit_side * j as f32;
                let y_pos = unit_side * i as f32;
                let cell = Rect {
                    x: x_pos,
                    y: y_pos,
                    w: side,
                    h: side,
                };

                if is_mouse_button_pressed(MouseButton::Left) {
                    if is_cell_clicked(x_pos, y_pos, side) {
                        color = GREEN;
                        if let Some(_selected_cell) = self.selected_cell {
                            if let Some(old_index) = self.selected_piece_index {
                                let piece_to_move = &board[old_index.0][old_index.1].take();

                                board[i][j] = piece_to_move.to_owned();
                                // x_pos = selected_cell.0;
                                // y_pos = selected_cell.1;
                            }
                        } else {
                            self.selected_cell = Some((x_pos, y_pos));
                            self.selected_piece_index = Some((i, j))
                        }
                    }
                }
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

fn is_cell_clicked(x: f32, y: f32, side: f32) -> bool {
    let (mouse_x, mouse_y) = mouse_position();
    mouse_x > x && mouse_x < x + side && mouse_y > y && mouse_y < y + side
}
