use bevy::{prelude::*, window::PrimaryWindow, math::vec2};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CursorWorldPos(vec2(0.0, 0.0)))
            .add_systems(Startup, hide_cursor)
            .add_systems(PreUpdate, set_cursor_world_pos)
            .add_systems(Update, update_cursor_sprite)
        ;
    }
}

#[derive(Resource)]
pub struct CursorWorldPos(pub Vec2);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CustomCursor;

fn update_cursor_sprite (
    mut cursor_transform_q: Query<&mut Transform, With<CustomCursor>>,
    cursor_world_pos: Res<CursorWorldPos>
) {
    for mut transform in cursor_transform_q.iter_mut() {
        transform.translation = cursor_world_pos.0.extend(1.0);     
    }
}

fn set_cursor_world_pos(
    mut cursor_world_pos: ResMut<CursorWorldPos>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_q.get_single()
        .expect("Exactly one Main Camera was not found");
    let window = window_q.get_single()
        .expect("Exactly one window was not found");

    if let Some(mouse_pos) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate()
    ) {
        cursor_world_pos.0 = mouse_pos;
    }    
}

fn hide_cursor(
    mut window_q: Query<&mut Window, With<PrimaryWindow>>
) {
    let mut window = window_q.get_single_mut()
        .expect("Exactly one window was not found");
    
    window.cursor.visible = false;
}
