use bevy::prelude::*;

pub mod billboard;

pub struct UtilityPlugin;
impl Plugin for UtilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, billboard::billboard_rotation);
    }
}
