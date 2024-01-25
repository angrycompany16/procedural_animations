use bevy::{prelude::*, sprite::MaterialMesh2dBundle, render::view::RenderLayers, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use bevy_mod_outline::*;
// use bevy_simple_2d_outline::*;

pub mod top_down_crawler;
pub mod cursor;
pub mod easing_functions;
pub mod builders;
pub mod render_shadows;

use top_down_crawler::*;
use cursor::*;
use render_shadows::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(
                        1200.0,
                        800.0,
                    ),
                    ..default()
                }),
                ..default()
            }),
            TopDownCrawlerPlugin,
            CursorPlugin,
            WorldInspectorPlugin::default(),
            ShadowRenderTexturePlugin {
                screen_width: 1200,
                screen_height: 800,
                
                render_layer_index: 1,
            }
        ))
        .insert_resource(ClearColor(Color::rgb(0.75, 0.9, 0.8)))
        .insert_resource(Msaa::Sample4)
        .add_systems(PostStartup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    render_tex_layer: Res<RenderTexLayer>,
    // mut window_q: Query<&mut Window, With<PrimaryWindow>>,
) {
    commands.spawn((
        Camera2dBundle::default(),
        RenderLayers::layer(**render_tex_layer),
        MainCamera,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.95, 0.95, 0.9))),
            ..default()
        },
        CustomCursor,
    ));
}