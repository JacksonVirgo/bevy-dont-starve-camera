use bevy::prelude::*;

pub mod player;
pub mod tree;

pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((player::PlayerPlugin, tree::TreePlugin));
    }
}
