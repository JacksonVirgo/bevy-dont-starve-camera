use bevy::prelude::*;

pub mod camera;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera::spawn_camera).add_systems(
            Update,
            (camera::camera_movement, camera::camera_scroll_zoom),
        );
    }
}
