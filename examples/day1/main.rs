use structopt::StructOpt;

use advent::args;

use bevy::{
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::RenderAssets,
        render_graph::{self, RenderGraph},
        render_resource::*,
        renderer::{RenderContext, RenderDevice},
        RenderApp, RenderStage,
    },
};

use std::borrow::Cow;

mod basic;

fn main() {
    let opt = args::Opt::from_args();

    if opt.compute {
        app(&opt);
        return;
    }

    let solution = if opt.part2 {
        basic::part2(&opt).unwrap()
    } else {
        basic::part1(&opt).unwrap()
    };
    println!("Solution [{}]: {}", if opt.part2 { 2 } else { 1 }, solution);
}

const SIZE: (u32, u32) = (320, 180);
const WORKGROUP_SIZE: u32 = 8;

fn app(opt: &args::Opt) {
    let content = std::fs::read_to_string(&opt.input).unwrap();

    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(InputFile(content))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                // uncomment for unthrottled FPS
                // present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            },
            ..default()
        }))
        .add_plugin(Day1ComputePlugin)
        .add_startup_system(setup)
        .run();
}

#[derive(Resource)]
pub struct InputFile(String);

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>, input: Res<InputFile>) {
    // Convert numbers into [u4;8]

    let content = input.0.clone();

    let mut as_bytes: Vec<_> = content
        .lines()
        .map(|line| {
            if line.is_empty() {
                [0; 4]
            } else {
                let as_num: i32 = line.parse().unwrap();
                let as_bytes = as_num.to_be_bytes();
                if as_bytes[0] != 0 {
                    panic!("Can't fit {} into 4 bytes got {:?}", as_num, as_bytes);
                }
                let shifted = [as_bytes[1], as_bytes[2], as_bytes[3], 255];
                shifted
            }
        })
        .flatten()
        .collect();

    let size = SIZE.0 * SIZE.1;
    if as_bytes.len() > size as usize {
        panic!("Need to increase the size of the images")
    }
    as_bytes.resize(size as usize * 4, 0);

    let mut image = Image::new(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        as_bytes,
        TextureFormat::Rgba8Unorm,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let image = images.add(image);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(640.0, 320.0)),
            ..default()
        },
        texture: image.clone(),
        ..default()
    });
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(Day1Image(image));
}

pub struct Day1ComputePlugin;

impl Plugin for Day1ComputePlugin {
    fn build(&self, app: &mut App) {
        // Extract the game of life image resource from the main world into the render world
        // for operation on by the compute shader and display on the sprite.
        app.add_plugin(ExtractResourcePlugin::<Day1Image>::default());
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<Day1Pipeline>()
            .add_system_to_stage(RenderStage::Queue, queue_bind_group);

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node("game_of_life", Day1Node::default());
        render_graph
            .add_node_edge(
                "game_of_life",
                bevy::render::main_graph::node::CAMERA_DRIVER,
            )
            .unwrap();
    }
}

#[derive(Resource, Clone, Deref, ExtractResource)]
struct Day1Image(Handle<Image>);

#[derive(Resource)]
struct Day1ImageBindGroup(BindGroup);

fn queue_bind_group(
    mut commands: Commands,
    pipeline: Res<Day1Pipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    game_of_life_image: Res<Day1Image>,
    render_device: Res<RenderDevice>,
) {
    let view = &gpu_images[&game_of_life_image.0];
    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &pipeline.texture_bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::TextureView(&view.texture_view),
        }],
    });
    commands.insert_resource(Day1ImageBindGroup(bind_group));
}

#[derive(Resource)]
pub struct Day1Pipeline {
    texture_bind_group_layout: BindGroupLayout,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

impl FromWorld for Day1Pipeline {
    fn from_world(world: &mut World) -> Self {
        let texture_bind_group_layout =
            world
                .resource::<RenderDevice>()
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::StorageTexture {
                            access: StorageTextureAccess::ReadWrite,
                            format: TextureFormat::Rgba8Unorm,
                            view_dimension: TextureViewDimension::D2,
                        },
                        count: None,
                    }],
                });
        let shader = world.resource::<AssetServer>().load("shaders/day1.wgsl");
        let mut pipeline_cache = world.resource_mut::<PipelineCache>();
        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: Some(vec![texture_bind_group_layout.clone()]),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
        });
        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: Some(vec![texture_bind_group_layout.clone()]),
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
        });

        Day1Pipeline {
            texture_bind_group_layout,
            init_pipeline,
            update_pipeline,
        }
    }
}

enum Day1State {
    Loading,
    Init,
    Update,
}

struct Day1Node {
    state: Day1State,
}

impl Default for Day1Node {
    fn default() -> Self {
        Self {
            state: Day1State::Loading,
        }
    }
}

impl render_graph::Node for Day1Node {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<Day1Pipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded, transition to the next stage
        match self.state {
            Day1State::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline)
                {
                    self.state = Day1State::Init;
                }
            }
            Day1State::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    self.state = Day1State::Update;
                }
            }
            Day1State::Update => {}
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let texture_bind_group = &world.resource::<Day1ImageBindGroup>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<Day1Pipeline>();

        let mut pass = render_context
            .command_encoder
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_bind_group(0, texture_bind_group, &[]);

        // select the pipeline based on the current state
        match self.state {
            Day1State::Loading => {}
            Day1State::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
            Day1State::Update => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
        }

        Ok(())
    }
}
