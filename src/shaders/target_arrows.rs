use super::*;
use crate::arrows::CorrectArrowEvent;
use crate::consts::*;
use crate::types::Directions::{self, *};

pub fn setup_target_arrows(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
    window: Res<WindowDescriptor>,
) {
    // Create a new shader pipeline
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("target_arrows.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("target_arrows.frag"),
        ))),
    }));

    render_graph.add_system_node(
        "last_time",
        RenderResourcesNode::<TimeSinceLastCorrect>::new(true),
    );
    render_graph
        .add_node_edge("last_time", base::node::MAIN_PASS)
        .unwrap();

    let directions = [Up, Down, Left, Right];
    for direction in directions.iter() {
        // Different z values so they don't overlap
        let z = match direction {
            Up => 0.3,
            Down => 0.4,
            Left => 0.5,
            Right => 0.6,
        };

        let mut transform =
            Transform::from_translation(Vec3::new(TARGET_POSITION, direction.y(), z));
        transform.scale = Vec3::new(300., 300., 1.);
        commands
            .spawn_bundle(SpriteBundle {
                render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                    pipeline_handle.clone(),
                )]),
                transform,
                visible: Visible {
                    is_transparent: true,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(TargetArrowSparkle {
                direction: *direction,
            })
            .insert(TimeSinceLastCorrect {
                last_time: -10.,
                points: 0.,
            })
            .insert(ShaderInputs {
                time: 0.,
                resolution: Vec2::new(window.width / window.height, 1.),
            });
    }
}

#[derive(RenderResources, TypeUuid)]
#[uuid = "c9400817-b3a3-4baa-8bfa-0320b9b87b17"]
pub struct TimeSinceLastCorrect {
    last_time: f32,
    points: f32,
}

pub struct TargetArrowSparkle {
    direction: Directions,
}

pub fn correct_arrow_event_listener(
    time: Res<Time>,
    mut correct_event_reader: EventReader<CorrectArrowEvent>,
    mut query: Query<(&TargetArrowSparkle, &mut TimeSinceLastCorrect)>,
) {
    for event in correct_event_reader.iter() {
        for (arrow, mut last_correct) in query.iter_mut() {
            if arrow.direction == event.direction {
                last_correct.last_time = time.seconds_since_startup() as f32;
                last_correct.points = event.points as f32 / 100.;
            }
        }
    }
}
