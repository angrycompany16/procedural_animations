use bevy::{prelude::*, math::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow, input::mouse};
use crate::cursor::*;

pub struct TopDownCrawlerPlugin {
    move_speed: f32,
}

impl Plugin for TopDownCrawlerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, control_vertebrae)
            .add_systems(Startup, spawn_crawler)
            .insert_resource(MoveSpeed(self.move_speed))
        ;
    }
}

impl Default for TopDownCrawlerPlugin {
    fn default() -> Self {
        TopDownCrawlerPlugin {
            move_speed: 100.0,
        }
    }
}

#[derive(Resource)]
struct MoveSpeed(f32);

#[derive(Component)]
pub struct Creature;

#[derive(Component)]
pub struct Controllable {
    vertebra_dist: f32,
}

#[derive(Component)]
pub struct Vertebra;

#[derive(Component)]
pub struct Head;

#[derive(Component)]
pub struct Leg;

pub fn spawn_crawler(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Top of spine, actively controlled by player 
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(25.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.9, 0.75, 0.75))),
            transform: Transform::from_translation(vec3(0.0, 60.0, 0.0)),
            ..default()
        },
        Vertebra,
        Controllable {
            vertebra_dist: 60.0,
        },
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(25.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.9, 0.75, 0.75))),
            transform: Transform::from_translation(vec3(0.0, 0.0, 0.0)),
            ..default()
        },
        Vertebra,
    ));
}

fn control_vertebrae(
    mut active_vertebra_q: Query<(&mut Transform, &Controllable)>,
    mut follower_vertebra_q: Query<
        &mut Transform, 
        (With<Vertebra>, Without<Controllable>)>,
    mouse_pos: Res<CursorWorldPos>,
    mouse_input: Res<Input<MouseButton>>,
    time: Res<Time>,
    move_speed: Res<MoveSpeed>,
) {
    if !mouse_input.pressed(MouseButton::Left) { return; }

    let (mut steer_transform, controllable) = active_vertebra_q.get_single_mut()
        .expect("At least one vertebra needs to have the Controllable component");

    let target_diff = mouse_pos.as_ref().0.extend(0.0) - steer_transform.translation;
    
    if target_diff.length() < 1.0 { return; }

    steer_transform.translation += target_diff.normalize() * move_speed.as_ref().0 * time.delta_seconds();

    let mut target_pos = steer_transform.translation;

    for mut follower_transform in follower_vertebra_q.iter_mut() {
        let diff = target_pos - follower_transform.translation;

        follower_transform.translation += (diff.length() - controllable.vertebra_dist) * diff.normalize();

        target_pos = follower_transform.translation;
    }
}


