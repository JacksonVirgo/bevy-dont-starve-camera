use bevy::{input::mouse::MouseWheel, prelude::*, render::camera::ScalingMode};

#[derive(Component)]
pub struct CameraController;

#[derive(Component)]
#[require(Transform)]
pub struct CameraFocus;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        CameraController,
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 20.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(20.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
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

pub fn camera_scroll_zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    mut q_camera: Query<&mut Projection, With<CameraController>>,
) {
    for ev in scroll_evr.read() {
        for mut proj in q_camera.iter_mut() {
            if let Projection::Orthographic(ref mut ortho) = *proj {
                let zoom_speed = 1.5;
                match ortho.scaling_mode {
                    ScalingMode::FixedVertical {
                        mut viewport_height,
                    } => {
                        viewport_height -= ev.y * zoom_speed;
                        viewport_height = viewport_height.clamp(5.0, 50.0);
                        ortho.scaling_mode = ScalingMode::FixedVertical {
                            viewport_height: viewport_height,
                        }
                    }
                    _ => {}
                };
            }
        }
    }
}
