use std::cell::RefMut;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, math::*, render::mesh::shape::Circle};
// use bevy_mod_outline::{OutlineBundle, OutlineVolume, OutlineMeshExt};
use crate::top_down_crawler::*;

pub struct FootBodyPartParams {
    pub size: f32,
    pub position: Vec2,
    
    pub foot_radius: f32,
    pub foot_offset: Vec2,
    pub step_length: f32,
    pub foot_z_index: f32,
    pub foot_speed: f32,
    pub foot_color: Color,
}

pub struct BodyPartParams {
    pub size: f32,
    pub position: Vec2,
}

pub struct HeadParams {
    pub size: Vec2,
    pub position: Vec2,
    
    pub turn_speed: f32,
    pub z_index: f32,
    pub neck_width: f32,
    
    pub vertebra_dist: f32,
    pub move_speed: f32,
    pub head_color: Color,

    pub eye_size: f32,
    pub pupil_size: f32,
}

pub fn spawn_vertebra_feet(
    params: FootBodyPartParams,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let mut mesh = Mesh::from(Circle::new(params.foot_radius));
    // mesh.generate_outline_normals().unwrap();
    // let mesh = meshes.add(shape::Circle::new(params.foot_radius).into());
    

    let foot_l = commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            material: materials.add(ColorMaterial::from(params.foot_color)),
            transform: Transform::from_translation(vec3(
                -params.foot_offset.x, 
                params.position.y, 
                params.foot_z_index
            )),
            ..default()
        },
        Foot {
            z_index: params.foot_z_index,
            foot_speed: params.foot_speed,

            target_pos: vec2(-params.foot_offset.x, params.position.y),
            grounded: true,
        },
        
        Name::new("Foot_l"),
    )).id();

    let foot_r = commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(params.foot_radius).into()).into(),
            material: materials.add(ColorMaterial::from(params.foot_color)),
            transform: Transform::from_translation(vec3(
                params.foot_offset.y, 
                params.position.y, 
                params.foot_z_index
            )),
            ..default()
        },
        Foot {
            z_index: params.foot_z_index,
            foot_speed: params.foot_speed,

            target_pos: vec2(params.foot_offset.x, params.position.y),
            grounded: true,
        },
        Name::new("Foot_r"),
    )).id();

    commands.spawn((
        Transform::from_translation(params.position.extend(0.0)),
        Vertebra {
            foot_offset: params.foot_offset,
            step_length: params.step_length,
            width: params.size,

            foot_l: Some(foot_l),
            foot_r: Some(foot_r),
        },
        Name::new("Vertebra"),
    ));
}

pub fn spawn_vertebra(
    params: BodyPartParams,
    commands: &mut Commands,
) {
    commands.spawn((
        Transform::from_translation(params.position.extend(0.0)),
        Vertebra {
            foot_offset: vec2(0.0, 0.0),
            step_length: 1.0,
            width: params.size,

            foot_l: None,
            foot_r: None,
        },
        Name::new("Vertebra"),
    ));
}

pub fn spawn_vertebra_many(
    params_vec: Vec<BodyPartParams>,
    commands: &mut Commands,
) {
    for params in params_vec {
        spawn_vertebra(params, commands);
    }
}

pub fn spawn_head(
    params: HeadParams,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let eye_black_l = commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(params.pupil_size).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.05, 0.05, 0.05))),
            transform: Transform::from_translation(vec3(0.0, params.eye_size / 2.0, 15.0)),
            ..default()
        },
        Name::new("Eye_black")
    )).id();

    let eye_l = commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(params.eye_size).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.95, 0.95, 0.9))),
            transform: Transform {
                translation: vec3(-params.size.x * 0.5, 0.0, 14.0),
                scale: vec3(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        Eye,
        Name::new("Eye_l")
    )).id();

    commands.entity(eye_l).add_child(eye_black_l);

    let eye_black_r = commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(params.pupil_size).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.05, 0.05, 0.05))),
            transform: Transform::from_translation(vec3(0.0, params.eye_size / 2.0, 15.0)),
            ..default()
        },
        Name::new("Eye_black")
    )).id();

    let eye_r = commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(params.eye_size).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.95, 0.95, 0.9))),
            transform: Transform {
                translation: vec3(params.size.x * 0.5, 0.0, 14.0),
                scale: vec3(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        Eye,
        Name::new("Eye_l")
    )).id();

    commands.entity(eye_r).add_child(eye_black_r);

    let head = commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(params.size.x * 0.5).into()).into(),
            material: materials.add(ColorMaterial::from(params.head_color)),
            transform: Transform {
                translation: vec3(params.position.x, params.position.y, params.z_index),
                scale: vec3(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        Head {
            z_index: params.z_index,
            neck_width: params.neck_width,
        },
        Controllable {
            move_speed: params.move_speed,
            turn_speed: params.turn_speed,
            vertebra_dist: params.vertebra_dist,
        },
        Name::new("Head"),
    )).id();

    commands.entity(head).add_child(eye_r);
    commands.entity(head).add_child(eye_l);
}