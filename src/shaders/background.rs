use super::*;

pub struct Background;
pub fn setup_background(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    window: Res<WindowDescriptor>,
) {
    // Create a new shader pipeline
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("background.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("background.frag"),
        ))),
    }));

    commands
        .spawn_bundle(SpriteBundle {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_scale(Vec3::new(
                window.width + 10.,
                window.height + 10.,
                1.,
            )),
            ..Default::default()
        })
        .insert(Background)
        .insert(ShaderInputs {
            time: 0.,
            resolution: Vec2::new(window.width / window.height, 1.),
        });
}

pub fn update_background_size(
    mut event_reader: EventReader<WindowResized>,
    mut background: Query<(&mut Transform, &Background)>,
) {
    for event in event_reader.iter() {
        for (mut transform, _) in background.iter_mut() {
            transform.scale = Vec3::new(event.width, event.height, 1.);
        }
    }
}
