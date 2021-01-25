use super::*;

pub struct Background;
pub fn setup_background(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
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

    let mut transform = Transform::from_scale(Vec3::new(810., 610., 1.));
    transform.translation = Vec3::new(0., 0., 10.);
    commands
        .spawn(SpriteBundle {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_scale(Vec3::new(810., 610., 1.)),
            ..Default::default()
        })
        .with(Background);
}

pub fn update_background_size(
    mut event_reader: Local<EventReader<WindowResized>>,
    events: Res<Events<WindowResized>>,
    mut background: Query<(&mut Transform, &Background)>,
) {
    for event in event_reader.iter(&events) {
        for (mut transform, _) in background.iter_mut() {
            transform.scale = Vec3::new(event.width, event.height, 1.);
        }
    }
}
