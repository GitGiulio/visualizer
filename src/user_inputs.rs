use bevy::prelude::*;
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::world::tile::*;
use crate::GameUpdate;
use crate::game_data::{GameData, MySet};
use crate::RobotAction::UpdateTile;

pub struct InputPlugin;

impl Plugin for InputPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update,go_stop)
            .add_systems(Update,destroy_test.in_set(MySet::First))//<--- rimuovere
            .add_systems(Update,back_pack_show_hide.in_set(MySet::First))
            .add_systems(Update,map_show_hide.in_set(MySet::First))
            .add_systems(Update,feed_show_hide.in_set(MySet::First));
    }
}

fn go_stop(keyboard_input: Res<Input<KeyCode>>, mut game_data: ResMut<GameData>){ ///USER INPUT VERO///

    if keyboard_input.pressed(KeyCode::Space) {
        game_data.autoplay = !game_data.autoplay;
    }
}
fn next(keyboard_input: Res<Input<KeyCode>>, mut game_data: ResMut<GameData>){ ///USER INPUT VERO///

    if keyboard_input.pressed(KeyCode::Right) && !game_data.autoplay{
        game_data.next += 1;
        if game_data.previous > 0 {
            game_data.previous -= 1;
        }

    }
}
fn previous(keyboard_input: Res<Input<KeyCode>>, mut game_data: ResMut<GameData>){ ///USER INPUT VERO///
    if keyboard_input.pressed(KeyCode::Left) && !game_data.autoplay{
        if game_data.next > 0 {
            game_data.next -= 1;
        }
        game_data.previous += 1;
    }
}
fn back_pack_show_hide(keyboard_input: Res<Input<KeyCode>>, mut game_data: ResMut<GameData>){ ///USER INPUT VERO///
    if keyboard_input.just_pressed(KeyCode::B) {
        game_data.robot_data.back_pack_visibility = !game_data.robot_data.back_pack_visibility;
    }
}
fn feed_show_hide(keyboard_input: Res<Input<KeyCode>>, mut game_data: ResMut<GameData>){ ///USER INPUT VERO///
    if keyboard_input.just_pressed(KeyCode::F) {
        game_data.feed_visibility = !game_data.feed_visibility;
    }
}
fn map_show_hide(keyboard_input: Res<Input<KeyCode>>, mut game_data: ResMut<GameData>){ ///USER INPUT VERO///
    if keyboard_input.just_pressed(KeyCode::M) {
        if game_data.camera_data.camera_mode != 3{
            game_data.camera_data.camera_mode_bu = game_data.camera_data.camera_mode;
            game_data.camera_data.camera_direction_bu = game_data.camera_data.camera_direction.clone();
            game_data.camera_data.camera_transform_bu = game_data.camera_data.camera_transform;
            game_data.camera_data.camera_velocity_bu = game_data.camera_data.camera_velocity;

            game_data.camera_data.camera_mode = 3;
            game_data.camera_data.camera_direction = crate::Direction::Up;
            game_data.camera_data.camera_transform = Transform::from_xyz(0.0,game_data.map_radius * 2.1,0.0).looking_at(Vec3::ZERO,Vec3::Z);
            game_data.camera_data.camera_velocity = Vec3::ZERO;
        }else {
            game_data.camera_data.camera_mode = game_data.camera_data.camera_mode_bu;
            game_data.camera_data.camera_direction = game_data.camera_data.camera_direction_bu.clone();
            game_data.camera_data.camera_transform = game_data.camera_data.camera_transform_bu;
            game_data.camera_data.camera_velocity = game_data.camera_data.camera_velocity_bu;
        }
    }
}
fn destroy_test(keyboard_input: Res<Input<KeyCode>>, mut game_update: ResMut<GameUpdate>){ // è solo per test, rimuovere
    if keyboard_input.just_pressed(KeyCode::Q){
        game_update.azioni.push(
            (UpdateTile{
                new_tile: Tile {
                    tile_type: TileType::DeepWater,
                    content: Content::Coin(1),
                    elevation: 0,
                },
                back_pack_update: vec![(Content::Garbage(0),12)],
                coordinates: (0.0, 0.0),
                energy: -110,
                points: 500.0,
            },WeatherType::Sunny)
        );
    }else if keyboard_input.just_pressed(KeyCode::W){
        game_update.azioni.push((
            crate::RobotAction::Move {
                direction: crate::Direction::Up,
                elevation: 0.0,
                energy: -10,
                points: 4.0,
            },WeatherType::Sunny)
        );
    }else if keyboard_input.just_pressed(KeyCode::A){
        game_update.azioni.push((
            crate::RobotAction::Move {
                direction: crate::Direction::Left,
                elevation: 0.0,
                energy:-10,
                points: 2.0,
            },WeatherType::Sunny)
        );
    }else if keyboard_input.just_pressed(KeyCode::S){
        game_update.azioni.push((
            crate::RobotAction::Move {
                direction: crate::Direction::Down,
                elevation: 0.0,
                energy: -10,
                points: 1.0,
            },WeatherType::Sunny)
        );
    }else if keyboard_input.just_pressed(KeyCode::D){
        game_update.azioni.push((
            crate::RobotAction::Move {
                direction: crate::Direction::Right,
                elevation: 0.0,
                energy: -10,
                points: 3.0,
            },WeatherType::Sunny)
        );
    }else if keyboard_input.just_pressed(KeyCode::E){
        game_update.azioni.push((
            crate::RobotAction::GainEnergy {
                energy: 0,
                points: 0.0,
            },WeatherType::TrentinoSnow)
        );
    }else if keyboard_input.just_pressed(KeyCode::G){
        game_update.azioni.push((
            crate::RobotAction::GainEnergy {
                energy: 1000,
                points: 0.0,
            },WeatherType::Sunny)
        );
    }else if keyboard_input.just_pressed(KeyCode::T){
        game_update.azioni.push((
            crate::RobotAction::Teleport {
                destination: (6.0,6.0),
                destination_elevation: 0.0,
                energy: -10,
                points: 3.0,
            },WeatherType::Sunny)
        );
    }
    //info!("agg {:?}",aggiornamento);
}