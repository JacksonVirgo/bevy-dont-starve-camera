#![cfg_attr(windows, windows_subsystem = "windows")]
use bevy::prelude::*;

use crate::camera;

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
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(4.0, 4.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            unlit: true,
            base_color: Color::WHITE,
            base_color_texture: Some(asset_server.load("ground.png")),
            ..default()
        })),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}

struct CorePlugins;
impl Plugin for CorePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(camera::CameraPlugin)
            .add_systems(Update, exit_on_esc)
            .add_systems(Startup, build_floor);
    }
}
