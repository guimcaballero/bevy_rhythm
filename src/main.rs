use bevy::{input::system::exit_on_esc_system, prelude::*};

mod arrows;
use arrows::ArrowsPlugin;
mod consts;
use consts::*;
mod types;
mod ui;
use ui::UIPlugin;
mod score;
use score::ScoreResource;
mod audio;
use audio::AudioPlugin;
mod shaders;
use shaders::ShadersPlugin;
mod menu;
use menu::MenuPlugin;
mod time;
use time::TimePlugin;
mod map_maker;
use map_maker::MapMakerPlugin;

fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .insert_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .insert_resource(WindowDescriptor {
            title: "Rhythm!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_state(AppState::Menu)
        .init_resource::<ScoreResource>()
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ArrowsPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(ShadersPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(TimePlugin)
        .add_plugin(MapMakerPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .commands()
        .spawn_bundle(UiCameraBundle::default());
}
