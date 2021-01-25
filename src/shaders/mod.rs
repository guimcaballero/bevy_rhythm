use bevy::render::render_graph::RenderResourcesNode;
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
    window::WindowResized,
};

mod background;
use background::*;

pub struct ShadersPlugin;
impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_background.system())
            .add_system(update_background_size.system());
    }
}
