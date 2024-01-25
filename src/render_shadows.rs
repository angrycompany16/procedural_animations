use bevy::{
    prelude::*,
    render::{camera::*, render_resource::*, view::RenderLayers,},
    math::*, sprite::{MaterialMesh2dBundle, Material2d, Material2dPlugin},
};
// TODO: Get basic shaders working
// use bevy_inspector_egui::egui::Shape;

#[derive(Resource, Deref, DerefMut)]
pub struct RenderTexLayer(u8);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/simple_shadow.wgsl".into()
    }
}

#[derive(Resource)]
pub struct ScreenDimensions {
    pub width: u32,
    pub height: u32,
}

pub struct ShadowRenderTexturePlugin {
    pub screen_width: u32,
    pub screen_height: u32,
    
    pub render_layer_index: u8
}

impl Plugin for ShadowRenderTexturePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(Material2dPlugin::<CustomMaterial>::default())
            .insert_resource(RenderTexLayer(self.render_layer_index))
            .insert_resource(ScreenDimensions {
                width: self.screen_width,
                height: self.screen_height,
            })
            .add_systems(Startup, setup)
        ;
        
    }
}

fn setup(
    mut commands: Commands,
    screen_dimensions: Res<ScreenDimensions>,
    render_layer_index: Res<RenderTexLayer>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let i_size = Extent3d {
        width: screen_dimensions.width, 
        height: screen_dimensions.height, 
        ..default()
    };
    
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: i_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(i_size);
    

    let image_handle = images.add(image);

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // Render before Render Tex camera
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));


    commands.spawn((
        // Change to materialmesh2dbundle
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::default().into()).into(),
            material: materials.add(CustomMaterial {
                color: Color::BLUE,
                color_texture: Some(image_handle),
            }),
            ..default()
        },
        RenderLayers::layer(render_layer_index.0)
    ));

}