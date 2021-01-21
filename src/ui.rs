use crate::ScoreResource;
use bevy::prelude::*;

fn setup_ui(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let material = color_materials.add(Color::NONE.into());

    commands
        // Time text node
        .spawn(NodeBundle {
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
                .spawn(TextBundle {
                    text: Text {
                        value: "Time: 0.0".to_string(),
                        font: font.clone(),
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(TimeText);
        })
        .spawn(NodeBundle {
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
                .spawn(TextBundle {
                    text: Text {
                        value: "Score: 0. Corrects: 0. Fails: 0".to_string(),
                        font,
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(ScoreText);
        });
}

struct TimeText;
fn update_time_text(time: Res<Time>, mut query: Query<(&mut Text, &TimeText)>) {
    // Song starts 3 seconds after real time
    let secs = time.seconds_since_startup() - 3.;

    // Don't do anything before the song starts
    if secs < 0. {
        return;
    }

    for (mut text, _tag) in query.iter_mut() {
        text.value = format!("Time: {:.2}", secs);
    }
}

struct ScoreText;
fn update_score_text(score: ChangedRes<ScoreResource>, mut query: Query<(&mut Text, &ScoreText)>) {
    for (mut text, _tag) in query.iter_mut() {
        text.value = format!(
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
        app.add_startup_system(setup_ui.system())
            .add_system(update_time_text.system())
            .add_system(update_score_text.system());
    }
}
