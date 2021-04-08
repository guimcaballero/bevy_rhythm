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
mod target_arrows;
use target_arrows::*;

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "0320b9b8-b3a3-4baa-8bfa-c94008177b17"]
/// Resource that will be passed to shaders
pub struct ShaderInputs {
    time: f32,
    resolution: Vec2,
}

/// Updates time in ShaderInputs every frame
fn update_time(time: Res<Time>, mut nodes: Query<&mut ShaderInputs>) {
    let time = time.seconds_since_startup();
    for mut node in nodes.iter_mut() {
        node.time = time as f32;
    }
}

/// Updates resolution in ShaderInputs if window size changes
fn update_resolution(
    mut event_reader: EventReader<WindowResized>,
    mut background: Query<&mut ShaderInputs>,
) {
    for event in event_reader.iter() {
        for mut node in background.iter_mut() {
            node.resolution = Vec2::new(event.width / event.height, 1.);
        }
    }
}

/// Adds ShaderInputs as an edge in the render graph
fn setup_render_graph(mut render_graph: ResMut<RenderGraph>) {
    render_graph.add_system_node("inputs", RenderResourcesNode::<ShaderInputs>::new(true));
    render_graph
        .add_node_edge("inputs", base::node::MAIN_PASS)
        .unwrap();
}

pub struct ShadersPlugin;
impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<ShaderInputs>()
            .add_asset::<TimeSinceLastCorrect>()
            .add_startup_system(setup_render_graph.system())
            .add_system(update_time.system())
            .add_system(update_resolution.system())
            .add_startup_system(setup_background.system())
            .add_system(update_background_size.system())
            .add_startup_system(setup_target_arrows.system())
            .add_system(correct_arrow_event_listener.system());
    }
}
