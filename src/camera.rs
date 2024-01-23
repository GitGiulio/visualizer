use bevy::prelude::*;
use crate::GameUpdate;
use crate::RobotAction;
use crate::game_data::{GameData, MySet};
use crate::Direction;
use crate::world::TileComponent;

#[derive(Component)]
pub struct Camera3DComponent;

pub struct CameraPlugin;

impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,spawn_camera)
            .add_systems(Update,change_camera)
            .add_systems(Update,camera_follow_robot.in_set(MySet::Third));
            //.add_systems(Update,camera_follow_mouse.in_set(MySet::Third)); TODO farlo meglio
    }
}

fn spawn_camera(mut commands: Commands,
                game_data: Res<GameData>
){
    commands.spawn((Camera3dBundle{
        transform: game_data.camera_data.camera_transform,
        ..default()
    }, Camera3DComponent));
}

fn change_camera(
    mut camera_query: Query<&mut Transform,With<Camera3DComponent>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut game_data: ResMut<GameData>,
    time: Res<Time>,
){
    let mut camera_transform = camera_query.single_mut();
    if keyboard_input.just_pressed(KeyCode::C){
        if game_data.camera_data.camera_mode == 0 { //CAMERA 7 di TM con visione libera TODO mouse?
            game_data.camera_data.camera_transform = Transform::from_xyz(game_data.robot_data.robot_translation.x, game_data.robot_data.robot_translation.y + 9.0, game_data.robot_data.robot_translation.z - 5.0).looking_at(Vec3::new(game_data.robot_data.robot_translation.x, 0.0, game_data.robot_data.robot_translation.z),Vec3::Z);
            *camera_transform = game_data.camera_data.camera_transform;
            game_data.camera_data.camera_mode = 1;
        }else if game_data.camera_data.camera_mode == 1{ //CAMERA 1 di TM già funzionante
            game_data.camera_data.camera_mode = 2;
            match game_data.camera_data.camera_direction {
                Direction::Right => {
                    game_data.camera_data.camera_transform = Transform::from_xyz(game_data.robot_data.robot_translation.x + 5.0, game_data.robot_data.robot_translation.y + 7.0, game_data.robot_data.robot_translation.z).looking_at(Vec3::new(game_data.robot_data.robot_translation.x, 0.0, game_data.robot_data.robot_translation.z),Vec3::Z);
                    //game_data.camera_data.camera_transform.rotate_y(f32::to_radians(90.0));
                }
                Direction::Left => {
                    game_data.camera_data.camera_transform = Transform::from_xyz(game_data.robot_data.robot_translation.x - 5.0, game_data.robot_data.robot_translation.y + 7.0, game_data.robot_data.robot_translation.z).looking_at(Vec3::new(game_data.robot_data.robot_translation.x, 0.0, game_data.robot_data.robot_translation.z),Vec3::Z);
                    game_data.camera_data.camera_transform.rotate_y(f32::to_radians(90.0));
                }
                Direction::Up => {
                    game_data.camera_data.camera_transform = Transform::from_xyz(game_data.robot_data.robot_translation.x, game_data.robot_data.robot_translation.y + 7.0, game_data.robot_data.robot_translation.z - 5.0).looking_at(Vec3::new(game_data.robot_data.robot_translation.x, 0.0, game_data.robot_data.robot_translation.z),Vec3::Z);
                }
                Direction::Down => {
                    game_data.camera_data.camera_transform = Transform::from_xyz(game_data.robot_data.robot_translation.x, game_data.robot_data.robot_translation.y + 7.0, game_data.robot_data.robot_translation.z + 5.0).looking_at(Vec3::new(game_data.robot_data.robot_translation.x, 0.0, game_data.robot_data.robot_translation.z),Vec3::Z);
                    //game_data.camera_data.camera_transform.rotate_y(f32::to_radians(180.0));
                }
            }
            *camera_transform = game_data.camera_data.camera_transform;

        }else if game_data.camera_data.camera_mode == 2{ //CAMERA 1 di TM già funzionante TODO si bugga se quando la metti il robot è girato (fixare)
            game_data.camera_data.camera_transform =  Transform::from_xyz(game_data.robot_data.robot_translation.x, game_data.robot_data.robot_translation.y + 10.0, game_data.robot_data.robot_translation.z).looking_at(Vec3::new(game_data.robot_data.robot_translation.x, 0.0, game_data.robot_data.robot_translation.z),Vec3::Z);
            *camera_transform = game_data.camera_data.camera_transform;
            game_data.camera_data.camera_mode = 0;
        }
    }
    if keyboard_input.pressed(KeyCode::V) && (game_data.camera_data.camera_mode == 0 || game_data.camera_data.camera_mode == 3){
        camera_transform.translation.y += 12.0 * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::X) && (game_data.camera_data.camera_mode == 0 || game_data.camera_data.camera_mode == 3){
        camera_transform.translation.y -= 12.0 * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::K) && (game_data.camera_data.camera_mode == 0 || game_data.camera_data.camera_mode == 3){
        camera_transform.translation.x -= 12.0 * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::H) && (game_data.camera_data.camera_mode == 0 || game_data.camera_data.camera_mode == 3){
        camera_transform.translation.x += 12.0 * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::U) && (game_data.camera_data.camera_mode == 0 || game_data.camera_data.camera_mode == 3){
        camera_transform.translation.z += 12.0 * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::J) && (game_data.camera_data.camera_mode == 0 || game_data.camera_data.camera_mode == 3){
        camera_transform.translation.z -= 12.0 * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::Y) && game_data.camera_data.camera_mode == 0 {
        camera_transform.rotate_x(f32::to_degrees(1.0));
    }
    if keyboard_input.pressed(KeyCode::I) && game_data.camera_data.camera_mode == 0{
        camera_transform.rotate_x(f32::to_degrees(1.0));
    }
    game_data.camera_data.camera_transform = *camera_transform;

}
fn camera_follow_mouse(
    mut camera_query: Query<(&Camera, &mut GlobalTransform),With<Camera3DComponent>>,
    windows: Query<&Window>,
    mut game_data: ResMut<GameData>,
) {
    let (camera, mut camera_transform) = camera_query.single_mut();
    let ground = &Transform::from_xyz(0.0,0.0,0.0);

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(&camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) = ray.intersect_plane(ground.translation, ground.up()) else {
        return;
    };
    let point = ray.get_point(distance);

    // Draw a circle just above the ground plane at that position.
    if game_data.camera_data.camera_mode == 0{
        game_data.camera_data.camera_transform = game_data.camera_data.camera_transform.looking_at(point,Vec3::Y);
        *camera_transform = GlobalTransform::from(game_data.camera_data.camera_transform);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn camera_follow_robot(
    mut camera_query: Query<&mut Transform,With<Camera3DComponent>>,
    mut game_data: ResMut<GameData>,
    game_update: Res<GameUpdate>,
){
    if !game_data.next_action{
        return;
    }else {
        let mut camera_transform = camera_query.single_mut();
        *camera_transform = game_data.camera_data.camera_transform;
        if game_data.camera_data.camera_mode == 0{
            return;
        }
        if game_update.azioni.len() > 0{
            match &game_update.azioni[0].0 {
                RobotAction::Move{direction,elevation,energy,points} => {
                    match direction {
                        Direction::Right => {
                            if game_data.camera_data.camera_mode != 3{
                                game_data.camera_data.camera_transform.translation = Transform::from_xyz(camera_transform.translation.x - 1.0, camera_transform.translation.y + elevation/10.0, camera_transform.translation.z).translation;
                                if game_data.camera_data.camera_mode == 2{
                                    game_data.camera_data.camera_transform.translation = Transform::from_xyz(game_data.robot_data.robot_translation.x + 5.0, camera_transform.translation.y + elevation/10.0, game_data.robot_data.robot_translation.z).translation;
                                    match game_data.camera_data.camera_direction {
                                        Direction::Right => {}
                                        Direction::Left => {
                                            game_data.camera_data.camera_transform.rotate_y(f32::to_radians(180.0));
                                        }
                                        Direction::Up => {
                                            game_data.camera_data.camera_transform.rotate_y(f32::to_radians(-90.0));
                                        }
                                        Direction::Down => {
                                            game_data.camera_data.camera_transform.rotate_y(f32::to_radians(90.0));
                                        }
                                    }
                                    *camera_transform = game_data.camera_data.camera_transform;
                                }
                                game_data.camera_data.camera_direction = Direction::Right;
                                game_data.camera_data.camera_velocity = Vec3::new(-1.0,elevation/10.0,0.0);
                            }else {
                                game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(camera_transform.translation.x - 1.0, camera_transform.translation.y + elevation/10.0, camera_transform.translation.z).translation;
                                if game_data.camera_data.camera_mode_bu == 2{
                                    game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(game_data.robot_data.robot_translation.x + 5.0, camera_transform.translation.y + elevation/10.0, game_data.robot_data.robot_translation.z).translation;
                                    match game_data.camera_data.camera_direction_bu {
                                        Direction::Right => {}
                                        Direction::Left => {
                                            game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(180.0));
                                        }
                                        Direction::Up => {
                                            game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(-90.0));
                                        }
                                        Direction::Down => {
                                            game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(90.0));
                                        }
                                    }
                                }
                                game_data.camera_data.camera_direction_bu = Direction::Right;
                                game_data.camera_data.camera_velocity_bu = Vec3::new(-1.0,elevation/10.0,0.0);
                            }
                        }
                        Direction::Left => {
                            if game_data.camera_data.camera_mode != 3 {
                                game_data.camera_data.camera_transform.translation = Transform::from_xyz(camera_transform.translation.x + 1.0, camera_transform.translation.y + elevation/10.0, camera_transform.translation.z).translation;
                                if game_data.camera_data.camera_mode == 2{
                                    game_data.camera_data.camera_transform.translation = Transform::from_xyz(game_data.robot_data.robot_translation.x - 5.0, camera_transform.translation.y + elevation/10.0, game_data.robot_data.robot_translation.z).translation;
                                    match game_data.camera_data.camera_direction {
                                        Direction::Right => {
                                            game_data.camera_data.camera_transform.rotate_y(f32::to_radians(180.0));
                                        }
                                        Direction::Left => {}
                                        Direction::Up => {
                                            game_data.camera_data.camera_transform.rotate_y(f32::to_radians(90.0));
                                        }
                                        Direction::Down => {
                                            game_data.camera_data.camera_transform.rotate_y(f32::to_radians(-90.0));
                                        }
                                    }
                                    *camera_transform = game_data.camera_data.camera_transform;
                                }
                                game_data.camera_data.camera_direction = Direction::Left;
                                game_data.camera_data.camera_velocity = Vec3::new(1.0, elevation / 10.0, 0.0);
                            }else {
                                game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(camera_transform.translation.x + 1.0, camera_transform.translation.y + elevation/10.0, camera_transform.translation.z).translation;
                                if game_data.camera_data.camera_mode_bu == 2{
                                    game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(game_data.robot_data.robot_translation.x - 5.0, camera_transform.translation.y + elevation/10.0, game_data.robot_data.robot_translation.z).translation;
                                    match game_data.camera_data.camera_direction_bu {
                                        Direction::Right => {
                                            game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(180.0));
                                        }
                                        Direction::Left => {}
                                        Direction::Up => {
                                            game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(90.0));
                                        }
                                        Direction::Down => {
                                            game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(-90.0));
                                        }
                                    }
                                }
                                game_data.camera_data.camera_direction_bu = Direction::Left;
                                game_data.camera_data.camera_velocity_bu = Vec3::new(1.0,elevation/10.0,0.0);
                            }
                        }
                        Direction::Up => {
                            if game_data.camera_data.camera_mode != 3 {
                                game_data.camera_data.camera_transform.translation = Transform::from_xyz(camera_transform.translation.x, camera_transform.translation.y + elevation/10.0, camera_transform.translation.z + 1.0).translation;
                                if game_data.camera_data.camera_mode == 2{
                                    game_data.camera_data.camera_transform.translation = Transform::from_xyz(game_data.robot_data.robot_translation.x, camera_transform.translation.y + elevation/10.0, game_data.robot_data.robot_translation.z - 5.0).translation;
                                    match game_data.camera_data.camera_direction {
                                        Direction::Right => {
                                            game_data.camera_data.camera_transform.rotate_y(f32::to_radians(90.0));
                                        }
                                        Direction::Left => {
                                            game_data.camera_data.camera_transform.rotate_y(f32::to_radians(-90.0));
                                        }
                                        Direction::Up => {}
                                        Direction::Down => {
                                            game_data.camera_data.camera_transform.rotate_y(f32::to_radians(180.0));
                                        }
                                    }
                                    *camera_transform = game_data.camera_data.camera_transform;
                                }
                                game_data.camera_data.camera_direction = Direction::Up;
                                game_data.camera_data.camera_velocity = Vec3::new(0.0, elevation / 10.0, 1.0);
                            }else {
                                game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(camera_transform.translation.x, camera_transform.translation.y + elevation/10.0, camera_transform.translation.z + 1.0).translation;
                                if game_data.camera_data.camera_mode_bu == 2{
                                    game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(game_data.robot_data.robot_translation.x, camera_transform.translation.y + elevation/10.0, game_data.robot_data.robot_translation.z - 5.0).translation;
                                    match game_data.camera_data.camera_direction_bu {
                                        Direction::Right => {
                                            game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(90.0));
                                        }
                                        Direction::Left => {
                                            game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(-90.0));
                                        }
                                        Direction::Up => {}
                                        Direction::Down => {
                                            game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(180.0));
                                        }
                                    }
                                }
                                game_data.camera_data.camera_direction_bu = Direction::Up;
                                game_data.camera_data.camera_velocity_bu = Vec3::new(0.0,elevation/10.0,1.0);
                            }
                        }
                        Direction::Down => {
                            if game_data.camera_data.camera_mode != 3{
                                game_data.camera_data.camera_transform.translation = Transform::from_xyz(camera_transform.translation.x, camera_transform.translation.y + elevation/10.0, camera_transform.translation.z - 1.0).translation;
                                if game_data.camera_data.camera_mode == 2{
                                    game_data.camera_data.camera_transform.translation = Transform::from_xyz(game_data.robot_data.robot_translation.x, camera_transform.translation.y + elevation/10.0, game_data.robot_data.robot_translation.z + 5.0).translation;
                                    match game_data.camera_data.camera_direction {
                                        Direction::Right => {
                                            game_data.camera_data.camera_transform.rotate_y(f32::to_radians(-90.0));
                                        }
                                        Direction::Left => {
                                            game_data.camera_data.camera_transform.rotate_y(f32::to_radians(90.0));
                                        }
                                        Direction::Up => {
                                            game_data.camera_data.camera_transform.rotate_y(f32::to_radians(180.0));
                                        }
                                        Direction::Down => {}
                                    }
                                    *camera_transform = game_data.camera_data.camera_transform;
                                }
                                game_data.camera_data.camera_direction = Direction::Down;
                                game_data.camera_data.camera_velocity = Vec3::new(0.0,elevation/10.0,-1.0);
                            }else {
                                game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(camera_transform.translation.x, camera_transform.translation.y + elevation/10.0, camera_transform.translation.z - 1.0).translation;
                                if game_data.camera_data.camera_mode_bu == 2{
                                    game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(game_data.robot_data.robot_translation.x, camera_transform.translation.y + elevation/10.0, game_data.robot_data.robot_translation.z + 5.0).translation;
                                    match game_data.camera_data.camera_direction_bu {
                                        Direction::Right => {
                                            game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(-90.0));
                                        }
                                        Direction::Left => {
                                            game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(90.0));
                                        }
                                        Direction::Up => {
                                            game_data.camera_data.camera_transform_bu.rotate_y(f32::to_radians(180.0));
                                        }
                                        Direction::Down => {}
                                    }
                                }
                                game_data.camera_data.camera_direction_bu = Direction::Down;
                                game_data.camera_data.camera_velocity_bu = Vec3::new(0.0,elevation/10.0,-1.0);
                            }
                        }
                    }
                },
                RobotAction::Teleport {destination,destination_elevation,energy,points} => {
                    if game_data.camera_data.camera_mode == 1{
                        camera_transform.translation = Transform::from_xyz(destination.0, camera_transform.translation.y + destination_elevation/10.0, destination.1 - 5.0).translation;
                        game_data.camera_data.camera_transform.translation = Transform::from_xyz(destination.0, camera_transform.translation.y + destination_elevation/10.0, destination.1 - 5.0).translation;
                    }else if game_data.camera_data.camera_mode == 2 {
                        match game_data.camera_data.camera_direction {
                            Direction::Right => {
                                game_data.camera_data.camera_transform.translation = Transform::from_xyz(destination.0 + 5.0, camera_transform.translation.y + destination_elevation / 10.0, destination.1).translation;
                            }
                            Direction::Left => {
                                game_data.camera_data.camera_transform.translation = Transform::from_xyz(destination.0 - 5.0, camera_transform.translation.y + destination_elevation / 10.0, destination.1).translation;
                            }
                            Direction::Up => {
                                game_data.camera_data.camera_transform.translation = Transform::from_xyz(destination.0, camera_transform.translation.y + destination_elevation / 10.0, destination.1 - 5.0).translation;
                            }
                            Direction::Down => {
                                game_data.camera_data.camera_transform.translation = Transform::from_xyz(destination.0, camera_transform.translation.y + destination_elevation / 10.0, destination.1 + 5.0).translation;
                            }
                        }
                        *camera_transform = game_data.camera_data.camera_transform;
                    }else if game_data.camera_data.camera_mode == 3 && game_data.camera_data.camera_mode != 0 {
                        if game_data.camera_data.camera_mode_bu == 1{
                            game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(destination.0, camera_transform.translation.y + destination_elevation / 10.0, destination.1 - 5.0).translation;
                        }else if game_data.camera_data.camera_mode_bu == 2 {
                            match game_data.camera_data.camera_direction {
                                Direction::Right => {
                                    game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(destination.0 + 5.0, camera_transform.translation.y + destination_elevation / 10.0, destination.1).translation;
                                }
                                Direction::Left => {
                                    game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(destination.0 - 5.0, camera_transform.translation.y + destination_elevation / 10.0, destination.1).translation;
                                }
                                Direction::Up => {
                                    game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(destination.0, camera_transform.translation.y + destination_elevation / 10.0, destination.1 - 5.0).translation;
                                }
                                Direction::Down => {
                                    game_data.camera_data.camera_transform_bu.translation = Transform::from_xyz(destination.0, camera_transform.translation.y + destination_elevation / 10.0, destination.1 + 5.0).translation;
                                }
                            }
                        }
                    }
                },
                _ => {}
            }


        }
    }
}