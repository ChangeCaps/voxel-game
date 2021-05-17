use quartz::prelude::*;
use noise::{NoiseFn, OpenSimplex};

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
    pub voxel_texture: Texture3d,
}

impl GameState {
    pub fn new(render_resource: &RenderResource) -> Self {
        let voxel_shader = Shader::load("assets/voxel.vert", "assets/voxel.frag").unwrap();
        let voxel_pipeline = RenderPipeline::new(
            PipelineDescriptor::default_settings(voxel_shader),
            render_resource,
        )
        .unwrap();

        let voxel_sampler = Sampler::new(&SamplerDescriptor::default(), render_resource);

        let mut voxel_texture = Texture3d::new(
            &TextureDescriptor::default_settings(D3::new(1024, 128, 1024)),
            render_resource,
        );

        let simplex = OpenSimplex::new();

        voxel_texture.write(render_resource, |data| {
            for x in 0..1024 {
                for z in 0..1024 {
                    let val = simplex.get([x as f64 / 100.0, z as f64 / 100.0]);
                    
                    for y in 0..128 {
                        let pos = Vec3::new(x as f32, y as f32, z as f32) / 10.0;

                        if val as f32 * 2.0 + 5.0 > pos.y {
                            data[x][y][z] = Color::rgb(0.7, 0.3, 0.2);
                        }
                    }
                }
            }
        });

        let mut camera = Camera::default();

        camera.transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);

        Self {
            voxel_pipeline,
            camera,
            voxel_sampler,
            voxel_texture,
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

        if ctx.window.cursor_grabbed && !ctx.window.cursor_visible {
            self.camera.angles += ctx.mouse.delta * 0.001;
            self.camera.transform.rotation =
                Quat::from_rotation_ypr(-self.camera.angles.x, -self.camera.angles.y, 0.0);
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
                    .bind("voxel_texture", self.voxel_texture.clone());
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
