#![allow(dead_code)]

use chess_board::ChessBoard;
use macroquad::prelude::*;

mod chess_board;
mod chess_piece;
mod fen;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(Color::new(0.4, 0.6, 0.3, 1.0));

        ChessBoard::new().draw_chess_board();

        if is_mouse_button_pressed(MouseButton::Left) {
            println!("{:?}", mouse_position());
        }

        next_frame().await
    }
}
