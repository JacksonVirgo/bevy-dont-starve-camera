#![cfg_attr(windows, windows_subsystem = "windows")]
use bevy::prelude::*;

pub fn setup_game() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Don't Starve Camera".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            CorePlugins,
        ))
        .insert_resource(ClearColor(Color::linear_rgb(0.0, 0.0, 0.0)))
        .run();
}

fn exit_on_esc(keyboard_input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

struct CorePlugins;
impl Plugin for CorePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(()).add_systems(Update, exit_on_esc);
    }
}
