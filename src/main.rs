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
use std::fmt::Debug;
use bevy::prelude::*;

use crate::assets_loader::AssetsLoaderPlugin;
use crate::camera::CameraPlugin;
use crate::game_data::{CameraData, GameData, GameDataPlugin, RobotData};
use crate::gui_overlay::GUIPlugin;
use crate::movement::MovementPlugin;
use crate::robot::RobotPlugin;
use crate::rudimental_a_i::ArtificialIntelligencePlugin;
use crate::user_inputs::InputPlugin;
use crate::weather::WeatherPlugin;
use crate::world::WorldPlugin;

use robotics_lib::runner::Runner;
use robotics_lib::runner::Runnable;
use robotics_lib::runner::{Robot};
use lazy_static::lazy_static;
use std::sync::{Mutex};
use std::collections::HashMap;
use rip_worldgenerator::MyWorldGen;
use robotics_lib::event::events::Event;
use robotics_lib::world::tile::{Content, Tile};

use std::io;
use bevy::prelude::Resource;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::world_generator::Generator;

// Static variables for data exchange between bevy and non bevy code
lazy_static! {
    // Store your variables here
    pub static ref points: Mutex<f32> = Mutex::new(0.00);
    pub static ref energy: Mutex<usize> = Mutex::new(0);
    pub static ref robot_view: Mutex<Vec<Vec<Option<Tile>>>> = Mutex::new(vec![]);
    pub static ref positions: Mutex<(usize, usize)> = Mutex::new((0, 0));
    pub static ref backpack_content: Mutex<HashMap<Content, usize>> = Mutex::new(HashMap::new());
    pub static ref events: Mutex<Vec<Event>> = Mutex::new(vec![]);
}


#[derive(Debug,Clone)]
pub(crate) enum Direction{
    Right,
    Left,
    Up,
    Down
}

fn from_map_to_option_world(map: &Vec<Vec<Tile>>)->Vec<Vec<Option<Tile>>>{ //Used to load the entire world for testing purpose
    let mut r = vec![];
    for i in 0..map.len(){
        let mut t = vec![];
        for j in 0..map.len(){
            t.push(Some(map[i][j].clone()));
        }
        r.push(t);

    }
    return r;
}


pub const ACTIONS_VELOCITY:f32 = 0.15;

pub struct VisualizerGLC;
impl VisualizerGLC{
    pub fn visualize_world(world_size: usize,world_bool: bool){
        let mut mondo;
        if !world_bool{
            let mut generator = MyWorldGen::new_param(world_size, 2, 2, 2, true, false, 3, false, None);
            mondo = generator.gen();
        }else {
            let mut generator = who_needs_gv_world_generator::WorldGenerator::new(world_size);
            mondo = generator.gen();
        }

        *robot_view.lock().unwrap() = from_map_to_option_world(&mondo.0);


        App::new()
            .insert_resource(GameData{
                autoplay:false,
                next:0,
                world_size,
                world: vec![vec![None;world_size];world_size],
                update_world: true,
                robot_data: RobotData::new(),
                camera_data : CameraData::new(),
                current_tile_elevation: 0.0,
                timer: Timer::from_seconds(ACTIONS_VELOCITY, TimerMode::Repeating),
                next_action: false,
                frames: 0,
                feed: vec![],
                feed_visibility: true,
                hided_content: (0.0, 0.0),
                content_visibility: true,
                ai:false,
                world_bool,
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
fn input_number() -> u32{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Errore durante la lettura dell'input.");

    // Converto l'input in un numero intero unsigned a 32 bit
    let _number: u32 = match input.trim().parse() {
        Ok(num) => {return num;} ,
        Err(_) => {
            println!("Input non valido, inserisci un numero intero.");
            return 0;
        }
    };
}

#[derive(Resource)]
pub(crate) struct RunnerTag(pub(crate) Runner);
unsafe impl Sync for RunnerTag {}
unsafe impl Send for RunnerTag {}

fn main(){

    let mut choice;
    let mut world_bool_bevy = false;

    println!("Choose a world: ");
    println!("1 - WhoNeedsGV");
    println!("2 - RustInPeace");

    let mut input_invalido = true;

    while input_invalido {
        choice = input_number();
        match choice {
            1 => {
                world_bool_bevy = true;
                input_invalido = false;
            }
            2 => {
                world_bool_bevy = false;
                input_invalido = false;
            }
            _ => {
                println!("invalid input");
            }
        }
    }
    VisualizerGLC::visualize_world(256,world_bool_bevy);
}
