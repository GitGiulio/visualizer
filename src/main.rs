mod camera;
mod robot;
mod world;
mod weather;
mod user_inputs;
mod assets_loader;
mod gui_overlay;
mod movement;
mod game_data;
mod rudimental_a_i;

use std::fmt::{Debug, Formatter};
use bevy::ecs::bundle::DynamicBundle;
use bevy::prelude::*;
use robotics_lib::world::environmental_conditions::{EnvironmentalConditions, WeatherType};
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
pub enum Direction{ //TODO capire come usarle comunque per la direzzione in cui deve guardare il robot nonostante non abbia gia la pappa pronta
    Right,
    Left,
    Up,
    Down
}
#[derive(Clone,Debug)]
pub enum RobotAction { //TODO EVENTI YEEEEE
    Move{direction:Direction,elevation:f32,energy:i32,points:f32},
    UpdateTile{new_tile:Tile,back_pack_update:Vec<(Content,i32)>,coordinates:(f32,f32),energy:i32,points:f32},
    DiscoverTile{tile:Tile,coordinates:(f32,f32),energy:i32,points:f32},
    GainEnergy{energy:i32,points:f32},
    Teleport{destination:(f32,f32),destination_elevation:f32,energy:i32,points:f32},
    Other{action_type:String,back_pack_update:Vec<(Content,i32)>,energy:i32,points:f32},
}
pub enum Event { //TODO trasferire a questa struttura dati tutte le funzioni (dovrebbe venire pure meglio)
    /// Robot has been initialized and its lifecycle has started
    Ready,
    /// Robot has ended its lifecycle
    Terminated,
    /// [robotics_lib::event::events::Event] fired when time of the day changes, contains the new [EnvironmentalConditions]
    TimeChanged(EnvironmentalConditions),

    /// [robotics_lib::event::events::Event] fired when the day changes, contains the new [EnvironmentalConditions]
    DayChanged(EnvironmentalConditions),

    /// [robotics_lib::event::events::Event] fired when energy gets recharged, contains the recharge amount
    EnergyRecharged(usize),

    /// [robotics_lib::event::events::Event] fired when energy is consumed, contains the consumed amount
    EnergyConsumed(usize),

    /// [robotics_lib::event::events::Event] fired when the robot moves to new coordinates
    ///
    /// This [robotics_lib::event::events::Event] contains the [Tile] to which the robot moved and the coordinates
    Moved(Tile, (usize, usize)),

    /// [robotics_lib::event::events::Event] fired when a tile content gets updated.
    ///
    /// This [robotics_lib::event::events::Event] contains the [Tile] of the updated content and the coordinates
    TileContentUpdated(Tile, (usize, usize)),

    /// [robotics_lib::event::events::Event] fired when a [Content] is added to the backpack, also contains the amount of content added
    AddedToBackpack(Content, usize),

    /// [robotics_lib::event::events::Event] fired when a [Content] is removed from the backpack, also contains the amount of content removed
    RemovedFromBackpack(Content, usize),
}
#[derive(Resource,Debug)] /// OGNi VOLTA CHE CAMBIA QUALCOSA L'IA MI AGGIORNA QUESTA RESOURCE E IO HO TUTTO LI PRONTO
pub struct GameUpdate{ //non so ancora bene come funziona rip
    pub azioni: Vec<(RobotAction,WeatherType)>, //sarà un Vec<Event> cosi goldo è  TODO discover tiles è un problema d** c***
    //pub points: f32, //i punti non sono trasmessi tramite eventi :(
}
pub struct VisualizerGLC;
impl VisualizerGLC{
    pub fn run<T:AI + Resource>(artificial_intelligence: T, robot_actions:Vec<(RobotAction,WeatherType)>, robot_spawn: (usize, usize), robot_elevation: usize,energy:usize,max_points:f32){
        let mut robot_data = RobotData::new();
        robot_data.energy = energy as i32;
        robot_data.robot_translation = Transform::from_translation(Vec3::new(robot_spawn.0 as f32,robot_elevation as f32 / 10.0 - 0.45,robot_spawn.1 as f32)).translation;
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
                content_visibility: true,
                max_points,
            })
            .insert_resource( artificial_intelligence)
            .insert_resource(GameUpdate {
                azioni: robot_actions,
            })
            .add_plugins(DefaultPlugins)
            //plugins developed by Giulio Lo Cigno
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
pub trait AI{
    fn next(&mut self)->Vec<(RobotAction,WeatherType)>;
}

#[derive(Resource,Debug)]
pub struct TestAI{
    dati:bool,
}
impl AI for TestAI{
    fn next(&mut self) -> Vec<(RobotAction, WeatherType)> { // TODO -> Vec<Event>
        return vec![];
    }
}

fn main() {
    let mut generator = rip_worldgenerator::MyWorldGen::new_param(100,1,1,1,false,false,4);

    //let mut generator = who_needs_gv_world_generator::WorldGenerator::new(150);

    // println!("e_seed {}", generator.get_e_seed()); //
    // println!("m_seed {}", generator.get_m_seed()); // get the seeds so u can recreate the same tile_map later if you need
    // println!("t_seed {}", generator.get_t_seed()); //

    let mut test_a_i = TestAI{ dati:true};
    let mut mondo = generator.gen();
    VisualizerGLC::run(test_a_i,from_map_to_action_vec(&mondo.0),mondo.1.clone(),mondo.0[mondo.1.0][mondo.1.1].elevation,7000,mondo.3);
}

fn from_map_to_action_vec(map: &Vec<Vec<Tile>>)->Vec<(RobotAction,WeatherType)>{
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
