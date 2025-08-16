use bevy::prelude::*;

#[derive(Component)]
pub struct CameraController;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        CameraController,
        Camera3d::default(),
        Transform::from_xyz(0.0, 4.5, -9.0) // behind origin
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));
}
