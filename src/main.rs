use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

pub mod top_down_crawler;
pub mod cursor;

use top_down_crawler::*;
use cursor::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TopDownCrawlerPlugin::default(),
            CursorPlugin,
        ))
        .insert_resource(ClearColor(Color::rgb(0.75, 0.9, 0.8)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // mut window_q: Query<&mut Window, With<PrimaryWindow>>,
) {
    commands.spawn((
        Camera2dBundle::default(),
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