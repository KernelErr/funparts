use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;

struct Animations(Vec<Handle<AnimationClip>>);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.4, 0.7, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .add_system(setup_scene_once_loaded)
        .run();
}

fn setup(
    mut commands: Commands,
    mut scene_spawner: ResMut<SceneSpawner>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(Animations(vec![
        asset_server.load("models/model.glb#Animation0"),
    ]));

    scene_spawner.spawn(asset_server.load("models/model.glb#Scene0"));
}

fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Ok(mut player) = player.get_single_mut() {
            player.play(animations.0[0].clone_weak()).repeat();
            *done = true;
        }
    }
}