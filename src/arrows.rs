use crate::consts::*;
use crate::time::ControlledTime;
use crate::types::*;
use crate::ScoreResource;
use bevy::prelude::*;

/// Keeps the textures and materials for Arrows
struct ArrowMaterialResource {
    red_texture: Handle<Image>,
    blue_texture: Handle<Image>,
    green_texture: Handle<Image>,
    border_texture: Handle<Image>,
}
impl FromWorld for ArrowMaterialResource {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let red_texture = asset_server.load("images/arrow_red.png");
        let blue_texture = asset_server.load("images/arrow_blue.png");
        let green_texture = asset_server.load("images/arrow_green.png");
        let border_texture = asset_server.load("images/arrow_border.png");
        ArrowMaterialResource {
            red_texture,
            blue_texture,
            green_texture,
            border_texture,
        }
    }
}

#[derive(Component)]
struct TargetArrow;

fn setup_target_arrows(mut commands: Commands, materials: Res<ArrowMaterialResource>) {
    use Directions::*;
    let directions = [Up, Down, Left, Right];

    for direction in directions.iter() {
        let mut transform =
            Transform::from_translation(Vec3::new(TARGET_POSITION, direction.y(), 1.));
        transform.rotate(Quat::from_rotation_z(direction.rotation()));
        commands
            .spawn_bundle(SpriteBundle {
                texture: materials.border_texture.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(140., 140.)),
                    ..Default::default()
                },
                transform,
                ..Default::default()
            })
            .insert(TargetArrow);
    }
}

/// Actual component that goes on the sprites
#[derive(Component)]
struct Arrow {
    speed: Speed,
    direction: Directions,
}

/// Spawns arrows
fn spawn_arrows(
    mut commands: Commands,
    mut song_config: ResMut<SongConfig>,
    materials: Res<ArrowMaterialResource>,
    time: Res<ControlledTime>,
) {
    // We get the current time since startup (secs) and the time since the last iteration (secs_last),
    // this way we check if any arrows should spawn in this window

    // Song starts 3 seconds after start, so we subtract 3 seconds
    let secs = time.seconds_since_startup() - 3.;
    let secs_last = secs - time.delta_seconds_f64();

    // Counter of how many arrows we need to spawn and remove from the list
    let mut remove_counter = 0;
    for arrow in &song_config.arrows {
        // List is ordered, so we can just check until an item fails
        // Check if arrow should be spawned at any point between last frame and this frame
        if secs_last < arrow.spawn_time && arrow.spawn_time < secs {
            remove_counter += 1;

            // Get the correct texture according to speed
            let texture = match arrow.speed {
                Speed::Slow => materials.red_texture.clone(),
                Speed::Medium => materials.blue_texture.clone(),
                Speed::Fast => materials.green_texture.clone(),
            };

            let mut transform =
                Transform::from_translation(Vec3::new(SPAWN_POSITION, arrow.direction.y(), 1.));
            // Rotate the arrow acording to direction
            transform.rotate(Quat::from_rotation_z(arrow.direction.rotation()));
            commands
                .spawn_bundle(SpriteBundle {
                    texture,
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(140., 140.)),
                        ..Default::default()
                    },
                    transform,
                    ..Default::default()
                })
                .insert(Arrow {
                    speed: arrow.speed,
                    direction: arrow.direction,
                });
        } else {
            break;
        }
    }

    // Remove the arrows we have spawned from the list
    for _ in 0..remove_counter {
        song_config.arrows.remove(0);
    }
}

/// Moves the arrows forward
fn move_arrows(time: Res<ControlledTime>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * arrow.speed.value();

        let distance_after_target = transform.translation.x - (TARGET_POSITION + THRESHOLD);
        if distance_after_target >= 0.02 {
            // Move the arrow down if it's past the target
            transform.translation.y -= time.delta_seconds() * distance_after_target * 2.;

            // Change the scale according to how far away the arrow is
            let scale = ((100. - distance_after_target / 3.) / 100.).max(0.2);
            transform.scale = Vec3::splat(scale);

            // Rotate the arrow according to distance and speed
            transform.rotate(Quat::from_rotation_z(
                -distance_after_target * arrow.speed.multiplier() / 460.,
            ));
        }
    }
}

pub struct CorrectArrowEvent {
    pub direction: Directions,
    pub points: usize,
}

/// Despawns arrows when they reach the end if the correct button is clicked
fn despawn_arrows(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Arrow)>,
    keyboard_input: Res<Input<KeyCode>>,
    mut score: ResMut<ScoreResource>,
    mut correct_arrow_events: EventWriter<CorrectArrowEvent>,
) {
    for (entity, transform, arrow) in query.iter() {
        let pos = transform.translation.x;

        // Check if arrow is inside clicking threshold
        if (TARGET_POSITION - THRESHOLD..=TARGET_POSITION + THRESHOLD).contains(&pos)
            && arrow.direction.key_just_pressed(&keyboard_input)
        {
            commands.entity(entity).despawn();

            let points = score.increase_correct(TARGET_POSITION - pos);

            correct_arrow_events.send(CorrectArrowEvent {
                direction: arrow.direction,
                points,
            });
        }

        // Despawn arrows after they leave the screen
        if pos >= 2. * TARGET_POSITION {
            commands.entity(entity).despawn();
            score.increase_fails();
        }
    }
}

pub struct ArrowsPlugin;
impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ArrowMaterialResource>()
            .add_event::<CorrectArrowEvent>()
            .add_system_set(
                SystemSet::on_enter(AppState::Game).with_system(setup_target_arrows.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(spawn_arrows.system())
                    .with_system(move_arrows.system())
                    .with_system(despawn_arrows.system()),
            );
    }
}
