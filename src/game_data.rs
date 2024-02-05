use bevy::prelude::*;
use bevy::utils::HashMap;
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::tile::Content::*;
use crate::RobotAction;

#[derive(Debug)]
pub(crate) struct RobotData{
    pub(crate) back_pack:HashMap<Content,i32>,
    pub(crate) back_pack_update:HashMap<Content,i32>,
    pub(crate) back_pack_visibility:u8,
    pub(crate) robot_velocity:Vec3,
    pub(crate) robot_direction:crate::Direction,
    pub(crate) robot_translation:Vec3,
    pub(crate) energy:i32,
    pub(crate) energy_update:i32,
    pub(crate) points:f32,
    pub(crate) points_update:f32,
}
impl RobotData{
    pub(crate) fn new()->Self{
        let mut back_pack = HashMap::new();
        back_pack.insert(Water(0),0);
        back_pack.insert(Tree(0),0);
        back_pack.insert(Rock(0),0);
        back_pack.insert(Fish(0),0);
        back_pack.insert(Coin(0),0);
        back_pack.insert(Bush(0),0);
        back_pack.insert(JollyBlock(0),0);
        back_pack.insert(Garbage(0),0);
        let mut back_pack_update = back_pack.clone();
        RobotData{
            back_pack,
            back_pack_update,
            back_pack_visibility: 1,
            robot_velocity: Vec3::ZERO,
            robot_direction: crate::Direction::Up,
            robot_translation: Vec3::ZERO,
            energy: 5000,
            energy_update: 0,
            points: 0.0,
            points_update: 0.0,
        }
    }
}
#[derive(Debug)]
pub(crate) struct CameraData{
    pub(crate) camera_mode:u8,
    pub(crate) camera_velocity:Vec3,
    pub(crate) camera_direction:crate::Direction,
    pub(crate) camera_transform:Transform,
    pub(crate) camera_mode_bu:u8,
    pub(crate) camera_velocity_bu:Vec3,
    pub(crate) camera_direction_bu:crate::Direction,
    pub(crate) camera_transform_bu:Transform,
}
impl CameraData{
    pub(crate) fn new()->Self{
        CameraData{
            camera_mode: 0,
            camera_velocity: Vec3::ZERO,
            camera_direction: crate::Direction::Up,
            camera_transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO,Vec3::Z),
            camera_mode_bu: 0,
            camera_velocity_bu: Vec3::ZERO,
            camera_direction_bu: crate::Direction::Up,
            camera_transform_bu: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO,Vec3::Z),
        }
    }
}
#[derive(Resource,Debug)]
pub(crate) struct GameData{
    pub(crate) autoplay:bool,
    pub(crate) next:usize,
    pub(crate) previous:usize,
    pub(crate) world:Vec<Vec<Option<Tile>>>,
    pub(crate) robot_data:RobotData,
    pub(crate) camera_data:CameraData,
    pub(crate) timer:Timer,
    pub(crate) next_action:bool,
    pub(crate) frames:usize,
    pub(crate) feed:Vec<robotics_lib::event::events::Event>,
    pub(crate) feed_visibility:bool,
    pub(crate) map_radius:f32,
    pub(crate) hided_content:(f32,f32),
    pub(crate) content_visibility:bool,
    pub(crate) max_points:f32,
    pub(crate) ai:bool,
}
impl GameData{
    pub fn get_autoplay(&self)->bool{
        self.autoplay
    }
    pub fn get_next(&self)->usize{
        self.next
    }
    pub fn get_previous(&self)->usize{
        self.previous
    }
    pub fn reduce_previous(&mut self){
        self.previous -= 1;
    }
    pub fn reduce_next(&mut self){
        self.next -= 1;
    }
}

#[derive(SystemSet,Debug,Hash,Eq, PartialEq,Clone)]
pub enum MySet{
    First,
    Second,
    Third,
    Fourth,
}

pub struct GameDataPlugin;

impl Plugin for GameDataPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (MySet::First,MySet::Second,MySet::Third,MySet::Fourth).chain(),
            )
            .insert_resource(ClearColor(Color::rgb(0.1,0.3,0.45)))
            .insert_resource(AmbientLight{
                color: Color::rgb(1.0, 1.0, 0.8),
                brightness: 1.0,
            })
            .add_systems(Update, update_game_data.in_set(MySet::Second));
    }
}

fn update_game_data(mut game_data: ResMut<GameData>,
                    time: Res<Time>,
){
    game_data.timer.tick(time.delta());
    game_data.frames += 1;
    if !game_data.timer.just_finished(){
        return;
    }else {
        info!("frames{}",game_data.frames);
        game_data.frames = 0;
        game_data.next_action = true;
        game_data.robot_data.robot_velocity = Vec3::ZERO;
        if game_data.camera_data.camera_mode != 3{
            game_data.camera_data.camera_velocity = Vec3::ZERO;
        }else {
            game_data.camera_data.camera_velocity_bu = Vec3::ZERO;
        }
    }
}