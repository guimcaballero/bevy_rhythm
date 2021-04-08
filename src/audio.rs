use crate::consts::*;
use crate::time::ControlledTime;
use crate::types::SongConfig;
use bevy::prelude::*;

fn start_song(audio: Res<Audio>, time: Res<ControlledTime>, config: Res<SongConfig>) {
    // Song starts 3 seconds after real time
    let secs = time.seconds_since_startup();
    let secs_last = secs - time.delta_seconds_f64();

    if secs_last <= 3. && 3. <= secs {
        audio.play(config.song_audio.clone());
    }
}

pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(start_song.system()));
    }
}
