use noise::{NoiseFn, OpenSimplex};
use quartz::prelude::*;
use rayon::prelude::*;

const DIMENSIONS: D2Array = D2Array::new(512, 512, 512);

#[derive(Default)]
pub struct Camera {
    pub projection: PerspectiveProjection,
    pub transform: Transform,
    pub angles: Vec2,
}

impl Camera {
    pub fn view_proj(&self) -> Mat4 {
        self.projection.matrix() * self.transform.matrix().inverse()
    }
}

pub struct GameState {
    pub voxel_pipeline: RenderPipeline,
    pub camera: Camera,
    pub voxel_sampler: Sampler,
    pub voxel_texture: Texture2dArray,
    pub generation_pipeline: RenderPipeline<format::Rgba8UnormSrgb>,
}

impl GameState {
    pub fn new(render_resource: &RenderResource) -> Self {
        let voxel_shader = Shader::load("assets/voxel.vert", "assets/voxel.frag").unwrap();
        let voxel_pipeline = RenderPipeline::new(
            PipelineDescriptor::default_settings(voxel_shader),
            render_resource,
        )
        .unwrap();

        let generation_shader =
            Shader::load("assets/generation.vert", "assets/generation.frag").unwrap();
        let generation_pipeline = RenderPipeline::<format::Rgba8UnormSrgb>::new(
            PipelineDescriptor::default_settings(generation_shader),
            render_resource,
        )
        .unwrap();

        let voxel_sampler = Sampler::new(&SamplerDescriptor::default(), render_resource);

        let voxel_texture = Texture2dArray::new(
            &TextureDescriptor::default_settings(DIMENSIONS),
            render_resource,
        );

        let mut camera = Camera::default();

        camera.transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);

        render_resource
            .render(|ctx| {
                generation_pipeline.bind_uniform("Radius", 0.5);

                for layer in 0..DIMENSIONS.layers {
                    let desc = RenderPassDescriptor {
                        color_attachments: vec![ColorAttachment {
                            texture: TextureAttachment::Texture(voxel_texture.layer_view(layer)),
                            resolve_target: None,
                            ops: Default::default(),
                        }],
                        ..Default::default()
                    };

                    generation_pipeline
                        .bind_uniform("Layer", layer as f32 / DIMENSIONS.layers as f32 * 2.0 - 1.0);

                    ctx.render_pass(&desc, &generation_pipeline).draw(0..6);
                }
            })
            .unwrap();

        Self {
            voxel_pipeline,
            camera,
            voxel_sampler,
            voxel_texture,
            generation_pipeline,
        }
    }
}

impl State for GameState {
    fn update(&mut self, ctx: UpdateCtx) -> Trans {
        if ctx.mouse.input.pressed(&MouseButton::Left) {
            ctx.window.cursor_grabbed = true;
            ctx.window.cursor_visible = false;
        }

        if ctx.keyboard.pressed(&Key::Escape) {
            ctx.window.cursor_grabbed = false;
            ctx.window.cursor_visible = true;
        }

        if !ctx.window.cursor_visible {
            self.camera.angles += ctx.mouse.delta * 0.001;
            self.camera.transform.rotation =
                Quat::from_rotation_ypr(-self.camera.angles.x, -self.camera.angles.y, 0.0);
        }

        if ctx.keyboard.pressed(&Key::R) {
            let shader = Shader::load("assets/voxel.vert", "assets/voxel.frag").unwrap();
            let pipeline = RenderPipeline::new(
                PipelineDescriptor::default_settings(shader),
                ctx.render_resource,
            );

            match pipeline {
                Ok(p) => self.voxel_pipeline = p,
                Err(e) => println!("Shader compile error: {}", e),
            }
        }

        let mut movement = Vec3::ZERO;

        if ctx.keyboard.held(&Key::W) {
            movement -= self.camera.transform.rotation * Vec3::Z;
        }

        if ctx.keyboard.held(&Key::S) {
            movement += self.camera.transform.rotation * Vec3::Z;
        }

        if ctx.keyboard.held(&Key::D) {
            movement += self.camera.transform.rotation * Vec3::X;
        }

        if ctx.keyboard.held(&Key::A) {
            movement -= self.camera.transform.rotation * Vec3::X;
        }

        self.camera.transform.translation += movement * ctx.delta_time * 100.0;
        self.camera.projection.aspect = ctx.window.aspect_ratio();

        Trans::None
    }

    fn render(&self, render_resource: &RenderResource) {
        render_resource
            .render(|ctx| {
                self.voxel_pipeline
                    .bind_uniform("Camera", self.camera.view_proj());
                self.voxel_pipeline
                    .bind_uniform("CameraPosition", self.camera.transform.translation);
                self.voxel_pipeline
                    .bind("voxel_texture", self.voxel_texture.view());
                self.voxel_pipeline
                    .bind("voxel_sampler", self.voxel_sampler.clone());

                ctx.render_pass(&RenderPassDescriptor::default(), &self.voxel_pipeline)
                    .draw(0..6);
            })
            .unwrap();
    }
}

fn main() {
    App::new().title("Voxel game").run(GameState::new).unwrap();
}
