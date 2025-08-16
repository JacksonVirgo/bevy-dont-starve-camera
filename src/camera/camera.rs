use bevy::prelude::*;

#[derive(Component)]
pub struct CameraController;

#[derive(Component)]
#[require(Transform)]
pub struct CameraFocus;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        CameraController,
        Camera3d::default(),
        Transform::from_xyz(0.0, 4.5, -9.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));
}

pub fn camera_movement(
    q_focus: Query<&Transform, With<CameraFocus>>,
    mut q_camera: Query<&mut Transform, (With<CameraController>, Without<CameraFocus>)>,
) {
    if let Ok(focus) = q_focus.single() {
        for mut camera in q_camera.iter_mut() {
            let offset = Vec3::new(0.0, 4.5, -9.0);
            camera.translation = camera.translation + offset;
            camera.look_at(focus.translation, Vec3::Y);
        }
    }
}
