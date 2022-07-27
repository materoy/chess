use macroquad::prelude::*;

#[derive(Debug, PartialEq)]
pub enum ChessPieceType {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
    None,
}

pub struct ChessPiece {
    pub color: ChessColor,
    pub piece_type: ChessPieceType,
    pub icon: Image,
}

impl ChessPiece {
    pub fn new(color: ChessColor, piece_type: ChessPieceType) -> Self {
        let color_prefix = color_prefix(&color);

        let icon_file_name = match piece_type {
            ChessPieceType::King => format!("{}_king.png", color_prefix),
            ChessPieceType::Queen => format!("{}_queen.png", color_prefix),

            ChessPieceType::Rook => format!("{}_queen.png", color_prefix),

            ChessPieceType::Knight => format!("{}_queen.png", color_prefix),

            ChessPieceType::Bishop => format!("{}_queen.png", color_prefix),

            ChessPieceType::Pawn => format!("{}_queen.png", color_prefix),

            ChessPieceType::None => String::from(""),
        };

        ChessPiece {
            color: color,
            piece_type: piece_type,
            icon: Image::from_file_with_format(
                &bytes_from_image(&icon_file_name),
                Some(ImageFormat::Png),
            ),
        }
    }

    pub fn empty() -> Self {
        ChessPiece {
            color: ChessColor::None,
            piece_type: ChessPieceType::None,
            icon: Image::empty(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ChessColor {
    Black,
    White,
    None,
}

fn color_prefix(color: &ChessColor) -> String {
    match color {
        ChessColor::Black => String::from("black"),
        ChessColor::White => String::from("white"),
        ChessColor::None => String::from(""),
    }
}

fn bytes_from_image(file_name: &str) -> Vec<u8> {
    let assets_folder = "assets/";
    let file_path = format!("{}{}", assets_folder, file_name);
    match std::fs::read(&file_path) {
        Ok(bytes) => bytes.into(),
        Err(err) => {
            eprintln!("Error: {} reading file: {}", err, &file_path);
            panic!()
        }
    }
}
