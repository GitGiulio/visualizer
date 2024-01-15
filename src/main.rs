mod camera;
mod robot;
mod world;
mod weather;
mod user_inputs;
mod assets_loader;
mod gui_overlay;
mod movement;
mod game_data;

use bevy::ecs::bundle::DynamicBundle;
use bevy::prelude::*;
use bevy::utils::HashMap;
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::tile::Content::*;
use robotics_lib::world::world_generator::Generator;
use crate::assets_loader::AssetsLoaderPlugin;
use crate::camera::CameraPlugin;
use crate::game_data::{CameraData, GameData, GameDataPlugin, RobotData};
use crate::gui_overlay::GUIPlugin;
use crate::movement::MovementPlugin;
use crate::robot::RobotPlugin;
use crate::user_inputs::InputPlugin;
use crate::weather::WeatherPlugin;
use crate::world::WorldPlugin;
#[derive(Debug,Clone)]
pub enum Direction{
    Right,
    Left,
    Up,
    Down
}
#[derive(Debug,Clone)]
pub enum RobotAction {
    Move{direction:Direction,elevation:f32,energy:i32,points:f32},
    UpdateTile{new_tile:Tile,back_pack_update:Vec<(Content,i32)>,coordinates:(f32,f32),energy:i32,points:f32},
    DiscoverTile{tile:Tile,coordinates:(f32,f32),energy:i32,points:f32},
    GainEnergy{energy:i32,points:f32},
    Teleport{destination:(f32,f32),destination_elevation:f32,energy:i32,points:f32},
    Craft{/*TODO dati necessari*/back_pack_update:Vec<(Content,i32)>,energy:i32,points:f32},
    Sell{/*TODO dati necessari*/back_pack_update:Vec<(Content,i32)>,energy:i32,points:f32},
    Buy{/*TODO dati necessari*/back_pack_update:Vec<(Content,i32)>,energy:i32,points:f32},
}
#[derive(Resource,Debug)] /// OGNi VOLTA CHE CAMBIA QUALCOSA L'IA MI AGGIORNA QUESTA RESOURCE E IO HO TUTTO LI PRONTO
pub struct GameUpdate{ //non so ancora bene come funziona rip
    pub azioni: Vec<(RobotAction,WeatherType)>, //TODO cambiarae con Rc<RefCell<Vec<(RobotAction,WeatherType)>>> e provare a vedere se panica
}

pub struct VisualizerGLC{
}
impl VisualizerGLC{
    pub fn run( robot_actions:Vec<(RobotAction,WeatherType)>, robot_spawn: (usize, usize), robot_elevation: usize,energy:usize){
        let mut robot_data = RobotData::new();
        robot_data.energy = energy as i32;
        robot_data.robot_translation = Transform::from_translation(Vec3::new(robot_spawn.0 as f32,robot_elevation as f32 / 10.0,robot_spawn.1 as f32)).translation;
        let mut camera_data= CameraData::new();
        camera_data.camera_transform = Transform::from_translation(Vec3::new(0.0,10.0,0.0)).looking_at(Vec3::ZERO,Vec3::Z);
        camera_data.camera_transform.translation = Transform::from_translation(Vec3::new(robot_spawn.0 as f32,(robot_elevation as f32 /10.0) + 10.0,robot_spawn.1 as f32)).translation;
        App::new()
            .insert_resource(GameData{
                autoplay:true,
                next:0,
                previous:0,
                robot_data,
                camera_data,
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                next_action: false,
                frames: 0,
                feed: vec![],
                feed_visibility: true,
                map_radius: 0.0,
                hided_content: (0.0, 0.0),
            })
            .insert_resource(GameUpdate {
                azioni: robot_actions,
            })
            .add_plugins(DefaultPlugins)
            //cose fatte da me
            .add_plugins(AssetsLoaderPlugin)
            .add_plugins(WorldPlugin)
            .add_plugins(WeatherPlugin)
            .add_plugins(GameDataPlugin)
            .add_plugins(MovementPlugin)
            .add_plugins(GUIPlugin)
            .add_plugins(RobotPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(InputPlugin)
            .run();
    }
}

fn main(){
    let mut generator = rip_worldgenerator::MyWorldGen::new_param(20,1,1,1,false,false,2);

    let mut mondo = generator.gen();

    VisualizerGLC::run(from_map_to_action_vec(mondo.0.clone()),mondo.1.clone(),mondo.0[mondo.1.0][mondo.1.1].elevation,7000);
}

fn from_map_to_action_vec(map:Vec<Vec<Tile>>)->Vec<(RobotAction,WeatherType)>{
    let mut r = vec![];
    for i in 0..map.len(){
        for j in 0..map.len(){
            r.push((RobotAction::DiscoverTile{
                tile:Tile{
                    tile_type: map[i][j].tile_type,
                    content: map[i][j].content.clone(),
                    elevation: map[i][j].elevation,
                },
                coordinates:(i as f32,j as f32),
                energy:-1,
                points:1.0,
            },WeatherType::Sunny));
        }
    }
    return r;
}
