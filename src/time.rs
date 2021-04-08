use crate::consts::*;
use bevy::{
    prelude::*,
    utils::{Duration, Instant},
};

pub struct ControlledTime {
    delta: Duration,
    last_update: Option<Instant>,
    delta_seconds_f64: f64,
    delta_seconds: f32,
    seconds_since_startup: f64,
    startup: Instant,
}
impl Default for ControlledTime {
    fn default() -> Self {
        Self {
            delta: Duration::from_secs(0),
            last_update: None,
            startup: Instant::now(),
            delta_seconds_f64: 0.0,
            seconds_since_startup: 0.0,
            delta_seconds: 0.0,
        }
    }
}

impl ControlledTime {
    pub fn reset_time(&mut self) {
        self.startup = Instant::now();
        self.seconds_since_startup = 0.0;
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.update_with_instant(now);
    }

    pub fn update_with_instant(&mut self, instant: Instant) {
        if let Some(last_update) = self.last_update {
            self.delta = instant - last_update;
            self.delta_seconds_f64 = self.delta.as_secs_f64();
            self.delta_seconds = self.delta.as_secs_f32();
        }

        let duration_since_startup = instant - self.startup;
        self.seconds_since_startup = duration_since_startup.as_secs_f64();
        self.last_update = Some(instant);
    }

    /// The delta between the current and last tick as [`f32`] seconds
    #[inline]
    pub fn delta_seconds(&self) -> f32 {
        self.delta_seconds
    }

    /// The delta between the current and last tick as [`f64`] seconds
    #[inline]
    pub fn delta_seconds_f64(&self) -> f64 {
        self.delta_seconds_f64
    }

    /// The time since startup in seconds
    #[inline]
    pub fn seconds_since_startup(&self) -> f64 {
        self.seconds_since_startup
    }
}

pub fn update_time(mut time: ResMut<ControlledTime>) {
    time.update();
}

pub fn reset_time_when_entering_game(mut time: ResMut<ControlledTime>) {
    time.reset_time();
}

pub struct TimePlugin;
impl Plugin for TimePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ControlledTime>()
            .add_system_set(
                SystemSet::on_enter(AppState::Game)
                    .with_system(reset_time_when_entering_game.system()),
            )
            .add_system_set(
                SystemSet::on_enter(AppState::MakeMap)
                    .with_system(reset_time_when_entering_game.system()),
            )
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(update_time.system()))
            .add_system_set(
                SystemSet::on_update(AppState::MakeMap).with_system(update_time.system()),
            );
    }
}
