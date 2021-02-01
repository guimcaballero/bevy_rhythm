use crate::consts::*;
use crate::time::ControlledTime;
use crate::types::{
    ArrowTimeToml,
    Directions::{self, *},
    Speed,
};
use bevy::{
    app::AppExit,
    input::{keyboard::KeyCode, Input},
    prelude::*,
};
use serde_derive::Serialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Debug, Default)]
struct Presses {
    arrows: Vec<ArrowTimeToml>,
}

fn save_key_presses(
    time: Res<ControlledTime>,
    keyboard_input: Res<Input<KeyCode>>,
    mut presses: ResMut<Presses>,
) {
    let directions = [Up, Down, Left, Right];
    for direction in directions.iter() {
        if direction.key_just_pressed(&keyboard_input) {
            presses.arrows.push(ArrowTimeToml {
                click_time: time.seconds_since_startup(),
                speed: Speed::Slow,
                direction: *direction,
            });
        }
    }
}

fn save_to_file_on_exit(
    mut event_reader: Local<EventReader<AppExit>>,
    events: Res<Events<AppExit>>,
    presses: Res<Presses>,
) {
    for _event in event_reader.iter(&events) {
        let text = toml::to_string(&*presses).expect("Couldn't convert to toml text");

        let mut file = File::create("map.toml").expect("Couldn't open map.toml");
        file.write_all(text.as_bytes())
            .expect("Couldn't write to map.toml");
    }
}

struct MapMakerArrow(Directions);
fn setup_map_maker_arrows(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: ResMut<AssetServer>,
) {
    let border_handle = materials.add(asset_server.load("images/arrow_border.png").into());

    let directions = [Up, Down, Left, Right];
    for direction in directions.iter() {
        let y = match direction {
            Up => 150.,
            Down => 50.,
            Left => -50.,
            Right => -150.,
        };

        let mut transform = Transform::from_translation(Vec3::new(0., y, 1.));
        transform.rotate(Quat::from_rotation_z(direction.rotation()));
        commands
            .spawn(SpriteBundle {
                material: border_handle.clone(),
                sprite: Sprite::new(Vec2::new(140., 140.)),
                transform,
                ..Default::default()
            })
            .with(MapMakerArrow(*direction));
    }
}

fn toggle_map_maker_arrows(
    mut query: Query<(&mut Visible, &MapMakerArrow)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut visible, arrow) in query.iter_mut() {
        visible.is_visible = arrow.0.key_pressed(&keyboard_input);
    }
}

struct MapMakerAudio(Handle<AudioSource>);
impl FromResources for MapMakerAudio {
    fn from_resources(resources: &Resources) -> Self {
        let asset_server = resources.get_mut::<AssetServer>().unwrap();
        let audio = asset_server.load("map_maker_song.mp3");
        Self(audio)
    }
}
fn start_song(audio: Res<Audio>, map_maker_audio: Res<MapMakerAudio>) {
    audio.play(map_maker_audio.0.clone());
}

pub struct MapMakerPlugin;
impl Plugin for MapMakerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Presses>()
            .init_resource::<MapMakerAudio>()
            .on_state_enter(APP_STATE_STAGE, AppState::MakeMap, start_song.system())
            .on_state_enter(
                APP_STATE_STAGE,
                AppState::MakeMap,
                setup_map_maker_arrows.system(),
            )
            .on_state_update(
                APP_STATE_STAGE,
                AppState::MakeMap,
                toggle_map_maker_arrows.system(),
            )
            .on_state_update(
                APP_STATE_STAGE,
                AppState::MakeMap,
                save_key_presses.system(),
            )
            .on_state_update(
                APP_STATE_STAGE,
                AppState::MakeMap,
                save_to_file_on_exit.system(),
            );
    }
}
