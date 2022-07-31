use macroquad::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum ChessPieceType {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

#[derive(Clone, Debug)]
pub struct ChessPiece {
    pub color: ChessColor,
    pub piece_type: ChessPieceType,
    pub icon: Image,
}

impl PartialEq for ChessPiece {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.piece_type == self.piece_type
    }

    fn ne(&self, other: &Self) -> bool {
        self.color != other.color || self.piece_type != self.piece_type
    }
}
impl ChessPiece {
    pub fn new(color: ChessColor, piece_type: ChessPieceType) -> Self {
        let color_prefix = color_prefix(&color);

        let icon_file_name = match piece_type {
            ChessPieceType::King => format!("{}_king.png", color_prefix),
            ChessPieceType::Queen => format!("{}_queen.png", color_prefix),

            ChessPieceType::Rook => format!("{}_rook.png", color_prefix),

            ChessPieceType::Knight => format!("{}_knight.png", color_prefix),

            ChessPieceType::Bishop => format!("{}_bishop.png", color_prefix),

            ChessPieceType::Pawn => format!("{}_pawn.png", color_prefix),
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

    pub fn to_piece_notation(&self) -> char {
        let repr_char = match self.piece_type {
            ChessPieceType::King => 'K',
            ChessPieceType::Queen => 'Q',
            ChessPieceType::Rook => 'R',
            ChessPieceType::Knight => 'N',
            ChessPieceType::Bishop => 'B',
            ChessPieceType::Pawn => 'P',
        };

        if self.color == ChessColor::Black {
            repr_char.to_ascii_lowercase()
        } else {
            repr_char
        }
    }

    pub fn is_move_valid(&self, row: usize, col: usize, dest_row: usize, dest_col: usize) -> bool {
        match self.piece_type {
            ChessPieceType::King => self.is_king_move_valid(row, col, dest_row, dest_col),
            ChessPieceType::Queen => self.is_queen_move_valid(row, col, dest_row, dest_col),
            ChessPieceType::Rook => self.is_rook_move_valid(row, col, dest_row, dest_col),
            ChessPieceType::Knight => self.is_knight_move_valid(row, col, dest_row, dest_col),
            ChessPieceType::Bishop => self.is_bishop_move_valid(row, col, dest_row, dest_col),
            ChessPieceType::Pawn => self.is_pawn_move_valid(row, col, dest_row, dest_col),
        }
    }

    fn is_king_move_valid(&self, row: usize, col: usize, dest_row: usize, dest_col: usize) -> bool {
        row.abs_diff(dest_row) == 1 || col.abs_diff(dest_col) == 1
    }

    fn is_queen_move_valid(
        &self,
        row: usize,
        col: usize,
        dest_row: usize,
        dest_col: usize,
    ) -> bool {
        row == dest_row || col == dest_col || row.abs_diff(dest_row) == col.abs_diff(dest_col)
    }

    fn is_rook_move_valid(&self, row: usize, col: usize, dest_row: usize, dest_col: usize) -> bool {
        row == dest_row || col == dest_col
    }

    fn is_knight_move_valid(
        &self,
        row: usize,
        col: usize,
        dest_row: usize,
        dest_col: usize,
    ) -> bool {
        (row.abs_diff(dest_row) == 1 && col.abs_diff(dest_col) == 2)
            || (row.abs_diff(dest_row) == 2 && col.abs_diff(dest_col) == 1)
    }

    fn is_bishop_move_valid(
        &self,
        row: usize,
        col: usize,
        dest_row: usize,
        dest_col: usize,
    ) -> bool {
        row.abs_diff(dest_row) == col.abs_diff(dest_col)
    }

    fn is_pawn_move_valid(&self, row: usize, col: usize, dest_row: usize, dest_col: usize) -> bool {
        match self.color {
            ChessColor::White => col == dest_col && row as isize - dest_row as isize == 1,
            ChessColor::Black => col == dest_col && dest_row as isize - row as isize == 1
            
        }
    }
}

pub trait Move {
    fn r#move(&self);
}

impl Move for ChessPiece {
    fn r#move(&self) {}
}

#[derive(Debug, PartialEq, Clone)]
pub enum ChessColor {
    Black,
    White,
}

fn color_prefix(color: &ChessColor) -> String {
    match color {
        ChessColor::Black => String::from("black"),
        ChessColor::White => String::from("white"),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_valid() {}
}
