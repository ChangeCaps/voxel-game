use quartz::prelude::*;

pub struct Camera {
    pub projection: PerspectiveProjection,
       
}

pub struct GameState {
    pub voxel_pipeline: RenderPipeline,
}

impl GameState {
    pub fn new(render_resource: &RenderResource) -> Self {
        let voxel_shader = Shader::load("assets/voxel.vert", "assets/voxel.frag").unwrap();
        let voxel_pipeline = RenderPipeline::new(
            PipelineDescriptor::default_settings(voxel_shader),
            render_resource,
        )
        .unwrap();

        Self { voxel_pipeline }
    }
}

impl State for GameState {
    fn render(&self, render_resource: &RenderResource) {
        render_resource
            .render(|ctx| {
                ctx.render_pass(&RenderPassDescriptor::default(), &self.voxel_pipeline);
            })
            .unwrap();
    }
}

fn main() {
    App::new().title("Voxel game").run(GameState::new).unwrap();
}
