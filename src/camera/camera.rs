use crate::camera::direction::CameraDirection;
use bevy::{input::mouse::MouseWheel, prelude::*};

#[derive(Component, Default)]
#[require(CameraDirection)]
pub struct CameraController;

#[derive(Component)]
#[require(Transform)]
pub struct CameraFocus;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        CameraController,
        CameraDirection::North,
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn camera_movement(
    q_focus: Query<&Transform, (With<CameraFocus>, Without<CameraController>)>,
    mut q_camera: Query<(&mut Transform, &CameraDirection), With<CameraController>>,
) {
    if let Ok(focus) = q_focus.single() {
        for (mut cam_tf, dir) in q_camera.iter_mut() {
            let r_xy = 10.0;
            let h = 10.0;
            cam_tf.translation = focus.translation + dir.offset(r_xy, h);
            cam_tf.look_at(focus.translation, Vec3::Y);
        }
    }
}

pub fn camera_direction_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut q_camera: Query<&mut CameraDirection, With<CameraController>>,
) {
    for mut direction in q_camera.iter_mut() {
        if keyboard.just_pressed(KeyCode::KeyQ) {
            println!("Q");
            *direction = direction.prev();
        }
        if keyboard.just_pressed(KeyCode::KeyE) {
            println!("E");
            *direction = direction.next();
        }
    }
}

pub fn camera_scroll_zoom(
    mut scroll: EventReader<MouseWheel>,
    mut q_cam: Query<(&mut Transform, &mut CameraDirection), With<CameraController>>,
    q_focus: Query<&Transform, (With<CameraFocus>, Without<CameraController>)>,
) {
    let speed = 1.0;
    let min = 5.0;
    let max = 30.0;

    let Ok(focus) = q_focus.single() else {
        return;
    };

    for ev in scroll.read() {
        for (mut cam_tf, dir) in q_cam.iter_mut() {
            // recover current h and r_xy
            let diff = cam_tf.translation - focus.translation;
            let h = diff.y;
            let r_xy = Vec2::new(diff.x, diff.z).length();

            // zoom target with clamped height, keep tilt by scaling r_xy proportionally
            let new_h = (h - ev.y * speed).clamp(min, max);
            let ratio = if h.abs() > 1e-4 { new_h / h } else { 1.0 };
            let new_r_xy = r_xy * ratio;

            cam_tf.translation = focus.translation + dir.offset(new_r_xy, new_h);
            cam_tf.look_at(focus.translation, Vec3::Y);
        }
    }
}
