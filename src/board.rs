use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, PickingEvent};

use crate::piece::{Piece, PieceColor};

#[derive(Component)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}

#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}

#[derive(Default)]
struct SelectedPiece {
    entity: Option<Entity>,
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .add_startup_system(create_board)
            .add_system(select_square)
            .add_system(color_squares);
    }
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add meshes and materials
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1.0 }));

    // spawn 64 squares
    for i in 0..8 {
        for j in 0..8 {
            // Plane
            commands
                .spawn_bundle(PbrBundle {
                    mesh: mesh.clone(),
                    material: if (i + j + 1) % 2 == 0 {
                        materials.add(Color::rgb(1.0, 0.9, 0.9).into())
                    } else {
                        materials.add(Color::rgb(0.0, 0.1, 0.1).into())
                    },
                    transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                    ..Default::default()
                })
                .insert_bundle(PickableBundle::default())
                .insert(Square { x: i, y: j });
        }
    }
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

fn color_squares(
    mut pick_events: EventReader<PickingEvent>,
    selected_square: ResMut<SelectedSquare>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Square, &Handle<StandardMaterial>)>,
) {
    let hover_color = Color::rgb(1.0, 0.0, 0.0);
    let selected_square_color = Color::rgb(0.0, 1.0, 0.0);
    let white_color = Color::rgb(1.0, 0.9, 0.9);
    let black_color = Color::rgb(0.0, 0.0, 0.0);

    // Get the entity under the cursor if there's one
    for event in pick_events.iter() {
        match event {
            PickingEvent::Selection(_) => {}
            PickingEvent::Hover(hover_event) => {
                for (entity, square, material_handle) in query.iter() {
                    // Get The actual material
                    let material = materials.get_mut(material_handle).unwrap();

                    match hover_event {
                        bevy_mod_picking::HoverEvent::JustEntered(square_under_cursor) => {
                            // Change the material color of square below the cursor
                            material.base_color = if &entity == square_under_cursor {
                                hover_color
                            } else if Some(entity) == selected_square.entity {
                                selected_square_color
                            } else if square.is_white() {
                                white_color
                            } else {
                                black_color
                            }
                        }
                        bevy_mod_picking::HoverEvent::JustLeft(_) => {}
                    };
                }
            }
            PickingEvent::Clicked(e) => {
                for (entity, square, material_handle) in query.iter() {
                    // Get The actual material
                    let material = materials.get_mut(material_handle).unwrap();

                    // Change the material color
                    material.base_color = if &entity == e {
                        selected_square_color
                    } else if Some(entity) == selected_square.entity {
                        selected_square_color
                    } else if square.is_white() {
                        white_color
                    } else {
                        black_color
                    }
                }
            }
        }
    }
}

fn select_square(
    mut pick_events: EventReader<PickingEvent>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    squares_query: Query<&Square>,
    mut pieces_query: Query<(Entity, &mut Piece)>,
) {
    // Get the square under the cursor and set it as selected
    for event in pick_events.iter() {
        match event {
            PickingEvent::Selection(_) => {}
            PickingEvent::Hover(_) => {}
            PickingEvent::Clicked(e) => {
                selected_square.entity = Some(*e);
                if let Ok(square) = squares_query.get(*e) {
                    if let Some(selected_piece_entity) = selected_piece.entity {
                        let pieces_vec = pieces_query.iter_mut().map(|(_, piece)| *piece).collect();
                        // Move the selected piece to the selected square
                        if let Ok((_piece_entity, mut piece)) =
                            pieces_query.get_mut(selected_piece_entity)
                        {
                            if piece.is_move_valid((square.x, square.y), pieces_vec) {
                                piece.x = square.x;
                                piece.y = square.y;
                            } else {
                                println!("Invalid move")
                            }
                        }
                        selected_square.entity = None;
                        selected_piece.entity = None
                    } else {
                        // Select the piece in the currently selected square
                        for (piece_entity, piece) in pieces_query.iter_mut() {
                            if piece.x == square.x && piece.y == square.y {
                                selected_piece.entity = Some(piece_entity);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn color_of_square(pos: (u8, u8), pieces: &Vec<Piece>) -> Option<PieceColor> {
    for piece in pieces {
        if piece.x == pos.0 && piece.y == pos.1 {
            return Some(piece.color);
        }
    }
    None
}

pub fn is_path_empty(begin: (u8, u8), end: (u8, u8), pieces: &Vec<Piece>) -> bool {
    // Same column
    if begin.0 == end.0 {
        for piece in pieces {
            if piece.x == begin.0
                && ((piece.y > begin.1 && piece.y < end.1)
                    || (piece.y > end.1 && piece.y < begin.1))
            {
                return false;
            }
        }
    }
    // Same row
    if begin.1 == end.1 {
        for piece in pieces {
            if piece.y == begin.1
                && ((piece.x > begin.0 && piece.x < end.0)
                    || (piece.x > end.0 && piece.x < begin.0))
            {
                return false;
            }
        }
    }

    // Diagonals
    let x_diff = (begin.0 as i8 - end.0 as i8).abs();
    let y_diff = (begin.1 as i8 - end.1 as i8).abs();
    if x_diff == y_diff {
        for i in 1..x_diff {
            let pos = if begin.0 < end.0 && begin.1 < end.1 {
                // left bottom - right top
                (begin.0 + i as u8, begin.1 + i as u8)
            } else if begin.0 < end.0 && begin.1 > end.1 {
                // left top - right bottom
                (begin.0 + i as u8, begin.1 - i as u8)
            } else if begin.0 > end.0 && begin.1 < end.1 {
                // right bottom - left top
                (begin.0 - i as u8, begin.1 + i as u8)
            } else {
                // begin.0 > end.0 && begin.1 > end.1
                // right top - left bottom
                (begin.0 - i as u8, begin.1 - i as u8)
            };

            if color_of_square(pos, pieces).is_some() {
                return false;
            }
        }
    }

    true
}
