use crate::utils::billboard::Billboard;
use bevy::prelude::*;

#[derive(Component)]
pub struct Tree;

fn spawn_tree(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("tree.png");

    let tile_size = 4.0;

    commands.spawn((
        Tree,
        Billboard,
        Mesh3d(meshes.add(Rectangle::new(tile_size, tile_size))),
        MeshMaterial3d(materials.add(StandardMaterial {
            unlit: true,
            base_color: Color::WHITE,
            base_color_texture: Some(texture_handle),
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
        Transform::from_xyz(-5.0, 2.0, 3.0),
        GlobalTransform::default(),
    ));
}
pub struct TreePlugin;
impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tree);
    }
}
