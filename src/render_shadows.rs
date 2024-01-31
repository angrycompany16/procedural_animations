use bevy::{
    prelude::*,
    render::{camera::*, render_resource::*, view::RenderLayers},
    math::*, sprite::{MaterialMesh2dBundle, Material2d, Material2dPlugin},
};

#[derive(Resource, Deref, DerefMut)]
pub struct RenderTexLayer(u8);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    background_color: Color,
    #[uniform(1)]
    shadow_color: Color,
    #[texture(2)]
    #[sampler(3)]
    screen_texture: Option<Handle<Image>>,
    #[uniform(4)]
    shadow_offset: Vec2,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/simple_drop_shadow.wgsl".into()
    }
}

#[derive(Resource)]
pub struct ScreenDimensions {
    pub width: u32,
    pub height: u32,
}

pub struct ShadowRenderTexturePlugin {
    pub pixel_scale_factor: f32,
    
    pub background_color: Color,
    pub shadow_color: Color,

    pub screen_width: u32,
    pub screen_height: u32,
    
    pub render_layer_index: u8
}

impl Plugin for ShadowRenderTexturePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<CustomMaterial>::default());
        
        let i_size = Extent3d {
            width: self.screen_width / (self.pixel_scale_factor as u32), 
            height: self.screen_height / (self.pixel_scale_factor as u32), 
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
    
        let mut images = app.world.get_resource_mut::<Assets<Image>>().unwrap();
        
        let image_handle = images.add(image);
    
        app.world.spawn((
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
    
        let mut meshes = app.world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh_handle = meshes.add(shape::Quad::new(vec2(self.screen_width as f32, self.screen_height as f32)).into()).into();

        let mut custom_materials = app.world.get_resource_mut::<Assets<CustomMaterial>>().unwrap();
        let custom_material_handle = custom_materials.add(CustomMaterial {
            background_color: self.background_color,
            shadow_color: self.shadow_color,
            screen_texture: Some(image_handle),
            shadow_offset: vec2(
                -10.0 / (self.screen_width as f32), 
                -10.0 / (self.screen_height as f32)
            ),
        });

        app.world.spawn((
            MaterialMesh2dBundle {
                mesh: mesh_handle,
                material: custom_material_handle,
                transform: Transform::from_scale(vec3(self.pixel_scale_factor, self.pixel_scale_factor, 1.0)),
                ..default()
            },
            RenderLayers::layer(self.render_layer_index)
        ));
    
        app.insert_resource(RenderTexLayer(self.render_layer_index));
    }
}