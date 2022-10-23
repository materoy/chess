extern crate bevy;
extern crate bevy_mod_picking;
mod board;
mod piece;
mod pieces;

use bevy::prelude::*;
use bevy_mod_picking::*;

fn main() {
    App::new()
        // Set anti aliasing to use 4 samples
        .insert_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .insert_resource(WindowDescriptor {
            title: "Chess".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        // .add_plugin(DebugCursorPickingPlugin)
        .add_plugin(PickingPlugin)
        .add_plugin(board::BoardPlugin)
        .add_plugin(pieces::PiecesPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 20.0, 4.0),
            )),
            ..Default::default()
        })
        .insert_bundle(PickingCameraBundle::default());

    // Light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 2400.0,
            radius: 40.0,
            shadows_enabled: false,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(4.0, 4.0, 4.0)),
        ..Default::default()
    });
    // TODO improve lighting
}
