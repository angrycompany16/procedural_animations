// TODO: add sprites to replace basic shapes


use std::f32::consts::PI;
use rand::Rng;

use bevy::{prelude::*, math::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, render::{render_resource::PrimitiveTopology, mesh::Indices}};
use crate::{cursor::*, builders::*};

const BODY_COLOR: Color = Color::rgb(0.3, 0.75, 0.35);
const DARK_BODY_COLOR: Color = Color::rgb(0.1, 0.6, 0.3);

pub struct TopDownCrawlerPlugin;

impl Plugin for TopDownCrawlerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (control_vertebrae, control_feet, lerp_feet, update_mesh, control_eyes))
            .add_systems(Startup, spawn_crawler)
            .add_systems(PostStartup, generate_mesh)
        ;
    }
}

#[derive(Component)]
pub struct Creature;

#[derive(Component)]
pub struct Controllable {
    pub move_speed: f32,
    pub turn_speed: f32,
    pub vertebra_dist: f32,
}

#[derive(Component)]
pub struct Vertebra {
    pub foot_offset: Vec2,
    pub step_length: f32,
    pub width: f32,

    pub foot_l: Option<Entity>,
    pub foot_r: Option<Entity>,
}

#[derive(Component)]
pub struct Head {
    pub z_index: f32,
    pub neck_width: f32,
}

#[derive(Component)]
pub struct Foot {
    pub foot_speed: f32,
    pub z_index: f32,

    pub target_pos: Vec2,
    pub grounded: bool,
}

#[derive(Component)]
pub struct Eye;

#[derive(Component)]
pub struct BodyMesh;

pub fn spawn_crawler(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let vertebra_spacing = 7.5;

    spawn_head(
        HeadParams {
            size: vec2(10.0, 10.0),
            position: vec2(0.0, vertebra_spacing),

            turn_speed: 2.0,
            z_index: 2.0,
            neck_width: 10.0,

            vertebra_dist: vertebra_spacing,
            move_speed: 87.5,
            head_color: BODY_COLOR,
        }, 
        &mut commands, 
        &mut meshes, 
        &mut materials
    );
    
    spawn_vertebra(
        BodyPartParams {
            size: 12.5,
            position: vec2(0.0, 0.0),
        },
        &mut commands
    );

    spawn_vertebra_feet(
        FootBodyPartParams {
            size: 15.0,
            position: vec2(0.0, -vertebra_spacing),
            
            foot_radius: 3.75,
            foot_offset: vec2(vertebra_spacing * 2.0, vertebra_spacing * 2.5),
            step_length: vertebra_spacing * 4.0,
            foot_z_index: -1.0,
            foot_speed: 4.5,
            foot_color: DARK_BODY_COLOR,
        },
        &mut commands, 
        &mut meshes, 
        &mut materials,
    );

    let body_sizes: Vec<f32> = 
        vec![12.5, 10.75, 10.0, 10.0, 10.75, 12.5];
    
    for (i, size) in body_sizes.iter().enumerate() {
        spawn_vertebra(
            BodyPartParams {
                size: *size,
                position: vec2(0.0, -((i as f32) + 2.0) * vertebra_spacing),
            },
            &mut commands
        );
    }

    spawn_vertebra_feet(
        FootBodyPartParams {
            size: 15.0,
            position: vec2(0.0, -8.0 * vertebra_spacing),
            
            foot_radius: 3.0,
            foot_offset: vec2(vertebra_spacing * 1.75, vertebra_spacing * 2.5),
            step_length: vertebra_spacing * 4.0,
            foot_z_index: -1.0,
            foot_speed: 4.5,
            foot_color: DARK_BODY_COLOR,
        },
        &mut commands, 
        &mut meshes, 
        &mut materials,
    );

    let tail_sizes: Vec<f32> = 
        vec![12.5, 10.0, 8.25, 6.25, 5.0, 15.0, 11.0, 7.0, 4.0, 2.0];
    
    for (i, size) in tail_sizes.iter().enumerate() {
        spawn_vertebra(
            BodyPartParams {
                size: *size,
                position: vec2(0.0, -((i as f32) + 9.0) * vertebra_spacing),
            },
            &mut commands
        );
    }
}

fn control_vertebrae(
    mut controllable_q: Query<(&mut Transform, &Controllable)>,
    mut follower_vertebra_q: Query<
        &mut Transform, 
        (With<Vertebra>, Without<Controllable>)>,
    mouse_pos: Res<CursorWorldPos>,
    mouse_input: Res<Input<MouseButton>>,
    time: Res<Time>,
) {
    if !mouse_input.pressed(MouseButton::Left) { return; }

    let (mut steer_transform, controllable) = controllable_q.get_single_mut()
        .expect("At least one body part needs to have the Controllable component");

    let target_diff = mouse_pos.as_ref().0.extend(0.0) - steer_transform.translation;
    let target_angle = target_diff.y.atan2(target_diff.x);

    if target_diff.length() < 4.0 { return; }

    steer_transform.translation += target_diff.normalize() * controllable.move_speed * time.delta_seconds();

    steer_transform.rotation = steer_transform.rotation.slerp(Quat::from_axis_angle(Vec3::Z, target_angle - PI * 0.5), time.delta_seconds() * controllable.turn_speed);

    let mut target_pos = steer_transform.translation;

    for mut follower_transform in follower_vertebra_q.iter_mut() {
        let diff = target_pos - follower_transform.translation;
        let angle = diff.y.atan2(diff.x);

        follower_transform.translation += (diff.length() - controllable.vertebra_dist) * diff.normalize();

        follower_transform.rotation = Quat::from_axis_angle(Vec3::Z, angle - PI * 0.5);

        target_pos = follower_transform.translation;
    }
}

fn control_feet(
    vertebra_q: Query<(&Transform, &Vertebra), Without<Foot>>,
    mut foot_q: Query<&mut Foot>,
    mut gizmos: Gizmos,
) {
    for (vertebra_transform, vertebra) in vertebra_q.iter() {
        let (Some(foot_l_), Some(foot_r_)) = (vertebra.foot_l, vertebra.foot_r) else { continue; };

        let Ok([mut foot_l, mut foot_r]) = foot_q.get_many_mut([foot_l_, foot_r_]) 
            else { continue; };

        let offset_pos_l = (vertebra_transform.translation
            + vertebra_transform.left() * vertebra.foot_offset.x
            + vertebra_transform.up() * vertebra.foot_offset.y).truncate();

        // gizmos.circle_2d(offset_pos_l, 5.0, Color::RED);
        
        if (offset_pos_l - foot_l.target_pos).length_squared() > vertebra.step_length.powf(2.0) && foot_r.grounded {
            foot_l.grounded = false;
            foot_l.target_pos = offset_pos_l;
        }

        // gizmos.circle_2d(foot_l.target_pos, 5.0, Color::GREEN);
        
        let offset_pos_r = (vertebra_transform.translation
            + vertebra_transform.right() * vertebra.foot_offset.x
            + vertebra_transform.up() * vertebra.foot_offset.y).truncate();

        // gizmos.circle_2d(offset_pos_r, 5.0, Color::RED);

        if (offset_pos_r - foot_r.target_pos).length_squared() > vertebra.step_length.powf(2.0) && foot_l.grounded {
            foot_r.grounded = false;
            foot_r.target_pos = offset_pos_r;
        }

        // gizmos.circle_2d(foot_r.target_pos, 5.0, Color::GREEN);
    }
}

fn lerp_feet(
    mut foot_q: Query<(&mut Transform, &mut Foot)>,
    time: Res<Time>,
) {
    for (mut transform, mut foot) in foot_q.iter_mut() {
        transform.translation = transform.translation.lerp(
            foot.target_pos.extend(foot.z_index), 
            time.delta_seconds() * foot.foot_speed
        );

        if (transform.translation.truncate() - foot.target_pos).length_squared() < 50.0 {
            foot.grounded = true;
        }
    }
}

fn control_eyes(
    head_q: Query<(&Children, &Transform), (With<Head>, Without<Eye>)>,
    mut eye_q: Query<(&mut Transform, &GlobalTransform), With<Eye>>,
    mut commands: Commands,
    mouse_pos: Res<CursorWorldPos>,
) {
    let Ok((children, head_transform)) = head_q.get_single() else { return; };

    for child in children.iter() {
        let Ok((mut transform, glob_transform)) = eye_q.get_mut(*child) else { continue; };

        let dir = mouse_pos.as_ref().0 - glob_transform.translation().truncate();
        let angle = dir.y.atan2(dir.x);
    
        transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, angle - head_transform.rotation.to_euler(EulerRot::XYZ).2 - PI * 0.5);
    }
}

fn generate_mesh(
    vertebra_q: Query<(&Transform, &Vertebra)>,
    head_q: Query<(&Transform, &Head)>,
    // mesh_q: Query<&Mesh2dHandle, With<BodyMesh>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let (mut next_transform, head) = head_q.single();
    let mut next_width = head.neck_width;
    let body_len = vertebra_q.iter().len();

    for (i, (transform, vertebra)) in vertebra_q.iter().enumerate() {
        let lower_l = transform.translation + transform.left() * vertebra.width * 0.5;
        let lower_r = transform.translation - transform.left() * vertebra.width * 0.5;

        let upper_l = next_transform.translation + next_transform.left() * next_width * 0.5;
        let upper_r = next_transform.translation - next_transform.left() * next_width * 0.5;

        positions.append(&mut vec![
            lower_l.to_array(), 
            lower_r.to_array(), 
            upper_l.to_array(), 
            upper_r.to_array()
        ]);

        uvs.append(&mut vec![
            [body_len as f32 - i as f32 - 1.0, 0.0], 
            [body_len as f32 - i as f32 - 1.0, 1.0], 
            [body_len as f32 - i as f32, 0.0], 
            [body_len as f32 - i as f32, 1.0], 
        ]);

        normals.append(&mut vec![
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ]);

        indices.append(&mut vec![
            0 + (4 * i) as u32, 
            1 + (4 * i) as u32, 
            3 + (4 * i) as u32,

            3 + (4 * i) as u32, 
            2 + (4 * i) as u32, 
            0 + (4 * i) as u32,
        ]);

        next_transform = transform;
        next_width = vertebra.width;
    }

    let mesh = Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION, 
            positions,
        )

        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0, 
            uvs,
        )

        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL, 
            normals,
        )

        .with_indices(Some(Indices::U32(indices))
    );

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            material: materials.add(ColorMaterial::from(BODY_COLOR)),
            transform: Transform::from_translation(vec3(0.0, 0.0, 10.0)),
            ..default()
        },
        BodyMesh,
    ));
}

fn update_mesh(
    vertebra_q: Query<(&Transform, &Vertebra)>,
    head_q: Query<(&Transform, &Head)>,
    mesh_q: Query<&Mesh2dHandle, With<BodyMesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut positions: Vec<[f32; 3]> = Vec::new();

    let (mut next_transform, head) = head_q.single();
    let mut next_width = head.neck_width;

    for (transform, vertebra) in vertebra_q.iter() {
        let lower_l = transform.translation + transform.left() * vertebra.width * 0.5;
        let lower_r = transform.translation - transform.left() * vertebra.width * 0.5;

        let upper_l = next_transform.translation + next_transform.left() * next_width * 0.5;
        let upper_r = next_transform.translation - next_transform.left() * next_width * 0.5;

        positions.append(&mut vec![
            lower_l.to_array(), 
            lower_r.to_array(), 
            upper_l.to_array(), 
            upper_r.to_array()
        ]);

        next_transform = transform;
        next_width = vertebra.width;
    }

    for mesh_handle in mesh_q.iter() {
        let Some(mesh) = meshes.get_mut(mesh_handle.0.id()) else { continue; };
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
    }
}