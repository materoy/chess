use macroquad::prelude::*;

enum ChessPieceType {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}
struct ChessPiece {
    piece_type: ChessPieceType,
    board_index: ((i32, i32), (i32, i32)),
}
struct ChessBoard {
    pieces: Vec<ChessPiece>,
    moves: Vec<String>,
}

impl ChessBoard {
    fn draw_chess_board() {
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
                draw_rectangle(x_pos, y_pos, side, side, color);
            }
        }
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(BEIGE);

        ChessBoard::draw_chess_board();

        if is_mouse_button_pressed(MouseButton::Left) {
            println!("{:?}", mouse_position());
        }

        next_frame().await
    }
}
