use crate::{
    camera::camera::{CameraController, CameraFocus},
    utils::billboard::Billboard,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("player.png"); // your sprite texture

    let tile_size = 4.0;

    commands.spawn((
        Player,
        Billboard,
        CameraFocus,
        Mesh3d(meshes.add(Rectangle::new(tile_size, tile_size))),
        MeshMaterial3d(materials.add(StandardMaterial {
            unlit: true,
            base_color: Color::WHITE,
            base_color_texture: Some(texture_handle),
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
        Transform::from_xyz(0.0, 2.0, 0.0),
        GlobalTransform::default(),
    ));
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    q_camera: Query<&Transform, With<CameraController>>,
    mut q_player: Query<&mut Transform, (With<Player>, Without<CameraController>)>,
    time: Res<Time>,
) {
    let Ok(cam_tf) = q_camera.single() else {
        return;
    };

    let mut dir_forward = cam_tf.forward().as_vec3();
    dir_forward.y = 0.0;
    if dir_forward.length_squared() < 1e-6 {
        dir_forward = Vec3::new(0.0, 0.0, -1.0);
    }
    let dir_forward = dir_forward.normalize();
    let dir_left = Vec3::new(dir_forward.z, 0.0, -dir_forward.x);

    let mut dir = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        dir += dir_forward;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        dir -= dir_forward;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        dir -= dir_left;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        dir += dir_left;
    }

    if dir.length_squared() == 0.0 {
        return;
    }
    let dir = dir.normalize();

    let speed = 5.0;
    for mut tf in q_player.iter_mut() {
        tf.translation += dir * speed * time.delta_secs();
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}
