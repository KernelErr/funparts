use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.4, 0.7, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_scene(asset_server.load("models/higokumaru/scene.gltf#Scene0"));
}