use bevy::prelude::*;

use crate::board::{color_of_square, is_path_empty};

#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

#[derive(Clone, Copy, Component)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    // Current position
    pub x: u8,
    pub y: u8,
}

impl Piece {
    // Returns possible positions that are available
    pub fn is_move_valid(&self, new_position: (u8, u8), pieces: Vec<Piece>) -> bool {
        // if there's a piece of the same color in the destination square it cant move
        if color_of_square(new_position, &pieces) == Some(self.color) {
            return false;
        }

        match self.piece_type {
            PieceType::King => {
                // Horizontal
                (self.x as i8 - new_position.0 as i8).abs() == 1 && (self.y == new_position.1) || 
                // Vertical 
                (self.y as i8 - new_position.1 as i8).abs() == 1 && (self.x == new_position.0) || 
                // Diagonal
                (self.x as i8 - new_position.0 as i8).abs() == 1 && (self.y as i8 - new_position.1 as i8).abs() == 1
            }
            PieceType::Queen => {
                is_path_empty((self.x, self.y), new_position, &pieces)  &&
                // Diagonal
                (self.x as i8 - new_position.0 as i8).abs() == (self.y as i8 - new_position.1 as i8).abs() ||
                // Horizontal
                (self.x == new_position.0 && self.y != new_position.1) ||
                // Vertical
                (self.x != new_position.0 && self.y == new_position.1)
            },
            PieceType::Rook => {
                is_path_empty((self.x, self.y), new_position, &pieces)  &&
                // Horizontal
                (self.x == new_position.0 && self.y != new_position.1) ||
                // Vertical
                (self.x != new_position.0 && self.y == new_position.1)
            },
            PieceType::Knight => {
                (self.x as i8 - new_position.0 as i8).abs() == 1 && (self.y as i8 - new_position.1 as i8).abs() == 2 ||
                (self.x as i8 - new_position.0 as i8).abs() == 2 && (self.y as i8 - new_position.1 as i8).abs() == 1 
            },
            PieceType::Bishop => {
                is_path_empty((self.x, self.y), new_position, &pieces)  &&
                // Diagonal
                (self.x as i8 - new_position.0 as i8).abs() == (self.y as i8 - new_position.1 as i8).abs() 
            },
            PieceType::Pawn => {
                match self.color {
                    PieceColor::White => {
                        // Normal move
                        if new_position.0 as i8 - self.x as i8 == 1 && self.y == new_position.1 {
                            if color_of_square((self.x, self.y), &pieces).is_none() {
                                return true;
                            }
                        } 

                        // First move
                        if self.x == 1 && new_position.0 as i8 - self.x as i8 == 2 && self.y == new_position.1  {
                            if color_of_square((self.x, self.y), &pieces).is_none() {
                                return true;
                            }
                        }

                        false
                    }

                    PieceColor::Black => {
                        // Normal move
                        if new_position.0 as i8 - self.x as i8 == -1 && self.y == new_position.1 {
                            if color_of_square((self.x, self.y), &pieces).is_none() {
                                return true;
                            }
                        } 

                        // First move
                        if self.x == 6 && new_position.0 as i8 - self.x as i8 == -2 && self.y == new_position.1  {
                            if color_of_square((self.x, self.y), &pieces).is_none() {
                                return true;
                            }
                        }

                        false
                    },
            }
            },
        }
    }
}
