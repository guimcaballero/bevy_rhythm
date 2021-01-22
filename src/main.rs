use bevy::{input::system::exit_on_esc_system, prelude::*};

mod arrows;
use arrows::ArrowsPlugin;
mod consts;
mod types;
mod ui;
use ui::UIPlugin;
mod score;
use score::ScoreResource;
mod audio;
use audio::AudioPlugin;

fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .add_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .add_resource(WindowDescriptor {
            title: "Rhythm!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .init_resource::<ScoreResource>()
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ArrowsPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(AudioPlugin)
        .run();
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let config = types::load_config("test.toml", &asset_server);

    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default())
        .insert_resource(config);
}
