use bevy::prelude::*;

use crate::camera::camera::CameraController;

#[derive(Component)]
pub struct Billboard;

pub fn billboard_rotation(
    mut q_billboard: Query<&mut Transform, With<Billboard>>,
    q_camera: Query<&Transform, (With<CameraController>, Without<Billboard>)>,
) {
    let Ok(camera_transform) = q_camera.single() else {
        return;
    };

    for mut transform in q_billboard.iter_mut() {
        transform.rotation = camera_transform.rotation;
    }
}
