use crate::types::SongConfig;
use bevy::prelude::*;

fn start_song(audio: Res<Audio>, time: Res<Time>, config: Res<SongConfig>) {
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
        app.add_system(start_song.system());
    }
}
