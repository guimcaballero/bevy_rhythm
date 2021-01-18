use bevy::{input::system::exit_on_esc_system, prelude::*};

mod arrows;
use arrows::ArrowsPlugin;
mod consts;
mod types;

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
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ArrowsPlugin)
        .run();
}

fn setup(commands: &mut Commands) {
    let config = types::load_config();

    commands
        .spawn(Camera2dBundle::default())
        .insert_resource(config);
}
