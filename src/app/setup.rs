#![cfg_attr(windows, windows_subsystem = "windows")]
use bevy::prelude::*;

use crate::{camera, entities};

pub fn setup_game() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Don't Starve Camera".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            CorePlugins,
        ))
        .insert_resource(ClearColor(Color::linear_rgb(0.0, 0.0, 0.0)))
        .run();
}

fn exit_on_esc(keyboard_input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

fn build_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let tile_size = 4.0;
    let grid_size = 10;

    let total_size = grid_size as f32 * tile_size;
    let offset = total_size / 2.0 - tile_size / 2.0;

    for x in 0..grid_size {
        for z in 0..grid_size {
            commands.spawn((
                Mesh3d(meshes.add(Rectangle::new(tile_size, tile_size))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    unlit: true,
                    base_color: Color::WHITE,
                    base_color_texture: Some(asset_server.load("grass.png")),
                    ..default()
                })),
                Transform::from_xyz(
                    x as f32 * tile_size - offset,
                    0.0,
                    z as f32 * tile_size - offset,
                )
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            ));
        }
    }
}

struct CorePlugins;
impl Plugin for CorePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((camera::CameraPlugin, entities::EntityPlugin))
            .add_systems(Update, exit_on_esc)
            .add_systems(Startup, build_floor);
    }
}
