use bevy::prelude::*;
use crate::assets_loader::SceneAssets;
use crate::GameUpdate;
use crate::RobotAction::*;
use crate::game_data::{GameData, MySet};

#[derive(Component,Debug)]
pub struct RobotComponent;
#[derive(Bundle)]
struct RobotBundle{
    model:SceneBundle,
}

pub struct RobotPlugin;

impl Plugin for RobotPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup,spawn_robot)
            .add_systems(Update,move_robot.in_set(MySet::Third))
            .add_systems(Update,get_energy.in_set(MySet::Third))
            .add_systems(Update,teleport_robot.in_set(MySet::Third));
    }
}
fn spawn_robot(mut commands: Commands,scene_assets: Res<SceneAssets>,
                game_data: Res<GameData>
){
    commands.spawn((RobotBundle{
        model:SceneBundle{
            scene: scene_assets.robot.clone(),
            transform: Transform::from_translation(game_data.robot_data.robot_translation).looking_at(Vec3::Z,game_data.robot_data.robot_translation),
            ..default()
        },
    }, RobotComponent));
}
fn get_energy(mut game_update: ResMut<GameUpdate>,
              mut game_data: ResMut<GameData>,
){
    if !game_data.next_action{
        return;
    }else {
        if game_update.azioni.len() != 0 {
            match &game_update.azioni[0].0 {
                GainEnergy{energy,points} => {
                    game_data.robot_data.points += points;
                    game_data.robot_data.points_update = *points;
                    game_data.robot_data.energy += energy;
                    game_data.robot_data.energy_update = *energy;
                }
                _ => {return;}
            }
        }
    }
}
fn move_robot(mut robot_query: Query<&mut Transform,With<RobotComponent>>,
              mut game_update: ResMut<GameUpdate>,
              mut game_data: ResMut<GameData>,
){
    if !game_data.next_action{
        return;
    }else {
        let mut robot_transform = robot_query.single_mut();
        robot_transform.translation = game_data.robot_data.robot_translation;
        if game_update.azioni.len() != 0 {
            match &game_update.azioni[0].0 {
                Move{direction,elevation,energy,points} => {
                    game_data.robot_data.points += points;
                    game_data.robot_data.points_update = *points;
                    game_data.robot_data.energy += energy;
                    game_data.robot_data.energy_update = *energy;
                    match direction {
                        crate::Direction::Right => {
                            game_data.robot_data.robot_translation = Transform::from_xyz(robot_transform.translation.x - 1.0, robot_transform.translation.y + elevation/10.0, robot_transform.translation.z).looking_at(Vec3::ZERO, Vec3::Z).translation;
                            match game_data.robot_data.robot_direction {
                                crate::Direction::Right => {}
                                crate::Direction::Left => { robot_transform.rotate_y(f32::to_radians(180.0));}
                                crate::Direction::Up => { robot_transform.rotate_y(f32::to_radians(-90.0));}
                                crate::Direction::Down => { robot_transform.rotate_y(f32::to_radians(90.0));}
                            }
                            game_data.robot_data.robot_direction = crate::Direction::Right;
                            game_data.robot_data.robot_velocity = Vec3::new(-1.0,elevation/10.0,0.0);
                        }
                        crate::Direction::Left => {
                            game_data.robot_data.robot_translation = Transform::from_xyz(robot_transform.translation.x + 1.0, robot_transform.translation.y + elevation/10.0, robot_transform.translation.z).looking_at(Vec3::ZERO, Vec3::Z).translation;
                            match game_data.robot_data.robot_direction {
                                crate::Direction::Right => { robot_transform.rotate_y(f32::to_radians(180.0));}
                                crate::Direction::Left => {}
                                crate::Direction::Up => { robot_transform.rotate_y(f32::to_radians(90.0));}
                                crate::Direction::Down => { robot_transform.rotate_y(f32::to_radians(-90.0));}
                            }
                            game_data.robot_data.robot_direction = crate::Direction::Left;
                            game_data.robot_data.robot_velocity = Vec3::new(1.0,elevation/10.0,0.0);
                        }
                        crate::Direction::Up => {
                            game_data.robot_data.robot_translation = Transform::from_xyz(robot_transform.translation.x, robot_transform.translation.y + elevation/10.0, robot_transform.translation.z + 1.0).looking_at(Vec3::ZERO, Vec3::Z).translation;
                            match game_data.robot_data.robot_direction {
                                crate::Direction::Right => { robot_transform.rotate_y(f32::to_radians(90.0));}
                                crate::Direction::Left => { robot_transform.rotate_y(f32::to_radians(-90.0));}
                                crate::Direction::Up => {}
                                crate::Direction::Down => { robot_transform.rotate_y(f32::to_radians(180.0));}
                            }
                            game_data.robot_data.robot_direction = crate::Direction::Up;
                            game_data.robot_data.robot_velocity = Vec3::new(0.0,elevation/10.0,1.0);
                        }
                        crate::Direction::Down => {
                            game_data.robot_data.robot_translation = Transform::from_xyz(robot_transform.translation.x, robot_transform.translation.y + elevation/10.0, robot_transform.translation.z - 1.0).looking_at(Vec3::ZERO, Vec3::Z).translation;
                            match game_data.robot_data.robot_direction {
                                crate::Direction::Right => { robot_transform.rotate_y(f32::to_radians(-90.0));}
                                crate::Direction::Left => { robot_transform.rotate_y(f32::to_radians(90.0));}
                                crate::Direction::Up => { robot_transform.rotate_y(f32::to_radians(180.0));}
                                crate::Direction::Down => {}
                            }
                            game_data.robot_data.robot_direction = crate::Direction::Down;
                            game_data.robot_data.robot_velocity = Vec3::new(0.0,elevation/10.0,-1.0);
                        }
                    }
                }
                _ => {
                }
            }

        }
    }
}
fn teleport_robot(mut robot_query: Query<&mut Transform,With<RobotComponent>>,
                  mut game_update: ResMut<GameUpdate>,
                  mut game_data: ResMut<GameData>,
){
    if !game_data.next_action{
        return;
    }else {
        if game_update.azioni.len() != 0 {
            match &game_update.azioni[0].0 {
                Teleport{destination,destination_elevation,energy,points} => {
                    let mut robot_transform = robot_query.single_mut();
                    robot_transform.translation = Transform::from_xyz(destination.0, robot_transform.translation.y + destination_elevation/10.0, destination.1).translation;
                    game_data.robot_data.robot_translation = Transform::from_xyz(destination.0, robot_transform.translation.y + destination_elevation/10.0, destination.1).translation;
                    game_data.robot_data.points += points;
                    game_data.robot_data.points_update = *points;
                    game_data.robot_data.energy += energy;
                    game_data.robot_data.energy_update = *energy;
                }
                _ => {return;}
            }

        }
    }
}
