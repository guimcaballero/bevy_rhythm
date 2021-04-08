use crate::consts::*;
use crate::time::ControlledTime;
use crate::ScoreResource;
use bevy::prelude::*;

fn setup_ui(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let material = color_materials.add(Color::NONE.into());

    commands
        // Time text node
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: material.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Time: 0.0",
                        TextStyle {
                            font_size: 40.0,
                            font: font.clone(),
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(TimeText);
        })
        .commands()
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.),
                    bottom: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            material,
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Score: 0. Corrects: 0. Fails: 0",
                        TextStyle {
                            font_size: 40.0,
                            font: font.clone(),
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(ScoreText);
        });
}

struct TimeText;

fn update_time_text(time: Res<ControlledTime>, mut query: Query<(&mut Text, &TimeText)>) {
    // Song starts 3 seconds after real time
    let secs = time.seconds_since_startup() - 3.;

    // Don't do anything before the song starts
    if secs < 0. {
        return;
    }

    for (mut text, _marker) in query.iter_mut() {
        text.sections[0].value = format!("Time: {:.2}", secs);
    }
}

struct ScoreText;
fn update_score_text(score: Res<ScoreResource>, mut query: Query<(&mut Text, &ScoreText)>) {
    if !score.is_changed() {
        return;
    }

    for (mut text, _marker) in query.iter_mut() {
        text.sections[0].value = format!(
            "Score: {}. Corrects: {}. Fails: {}",
            score.score(),
            score.corrects(),
            score.fails()
        );
    }
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_ui.system()))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(update_time_text.system())
                    .with_system(update_score_text.system()),
            );
    }
}
