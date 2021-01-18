use crate::consts::*;
use bevy::prelude::*;

/// Keeps the textures and materials for Arrows
struct ArrowMaterialResource {
    red_texture: Handle<ColorMaterial>,
    blue_texture: Handle<ColorMaterial>,
    green_texture: Handle<ColorMaterial>,
    border_texture: Handle<ColorMaterial>,
}
impl FromResources for ArrowMaterialResource {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = resources.get::<AssetServer>().unwrap();

        let red_handle = asset_server.load("images/arrow_red.png");
        let blue_handle = asset_server.load("images/arrow_blue.png");
        let green_handle = asset_server.load("images/arrow_green.png");
        let border_handle = asset_server.load("images/arrow_border.png");
        ArrowMaterialResource {
            red_texture: materials.add(red_handle.into()),
            blue_texture: materials.add(blue_handle.into()),
            green_texture: materials.add(green_handle.into()),
            border_texture: materials.add(border_handle.into()),
        }
    }
}

/// Actual component that goes on the sprites
struct Arrow;

/// Keeps track of when to Spawn a new arrow
struct SpawnTimer(Timer);

/// Spawns arrows
fn spawn_arrows(
    commands: &mut Commands,
    materials: Res<ArrowMaterialResource>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }

    let transform = Transform::from_translation(Vec3::new(SPAWN_POSITION, 0., 1.));
    commands
        .spawn(SpriteBundle {
            material: materials.red_texture.clone(),
            sprite: Sprite::new(Vec2::new(140., 140.)),
            transform,
            ..Default::default()
        })
        .with(Arrow);
}

/// Moves the arrows forward
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, _arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * BASE_SPEED;
    }
}

pub struct ArrowsPlugin;
impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ArrowMaterialResource>()
            .add_resource(SpawnTimer(Timer::from_seconds(1.0, true)))
            .add_system(spawn_arrows.system())
            .add_system(move_arrows.system());
    }
}
