use bevy::ecs::bundle::DynamicBundle;
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use robotics_lib::world::tile::*;
use crate::GameUpdate;
use crate::RobotAction::*;
use crate::game_data::{GameData,MySet};
use crate::assets_loader::SceneAssets;
use robotics_lib::world::tile::Content::*;
use robotics_lib::world::tile::TileType::*;

#[derive(Bundle)]
pub struct ContentBundle{
    model: SceneBundle,
}
#[derive(Component,Debug)]
pub struct TileComponent;
#[derive(Component,Debug)]
pub struct ContentComponent;
#[derive(Bundle)]
pub struct TileBundle{
    model: SceneBundle,
}
pub struct WorldPlugin;

impl Plugin for WorldPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup,create_world)
            .add_systems(Update, discover_tile.in_set(MySet::Third))
            .add_systems(Update,update_tile.in_set(MySet::Third))
            .add_systems(Update,update_content.in_set(MySet::Third))
            .add_systems(Update, hide_content_under_robot.in_set(MySet::Second))
            .add_systems(Update, remove_game_update.in_set(MySet::Fourth));
    }
}

fn create_world(mut commands: Commands,
                scene_assets: Res<SceneAssets>,
                mut meshes: ResMut<Assets<Mesh>>,
                mut materials: ResMut<Assets<StandardMaterial>>,
                mut game_update: ResMut<GameUpdate>,
                mut game_data: ResMut<GameData>,
){
    while game_update.azioni.len() != 0 {
        match &game_update.azioni[0].0 {
            DiscoverTile{tile,coordinates,energy,points} => {
                let new_tile_radius = f32::sqrt((coordinates.0*coordinates.0) + (coordinates.1*coordinates.1));
                if new_tile_radius > game_data.map_radius {
                    game_data.map_radius = new_tile_radius;
                }
                let mut tile_scene;
                let mut tile_scale = Transform::from_scale(Vec3::new(0.5,0.5,0.5)).scale;
                let mut content_scene;
                let mut content_transform = Transform{
                    translation: Transform::from_xyz(coordinates.0,(tile.elevation as f32 / 10.0) - 1.5 ,coordinates.1).translation,
                    rotation: Default::default(),
                    scale: Transform::from_scale(Vec3::new(0.1,0.1,0.1)).scale,
                };
                match tile.tile_type {
                    DeepWater => { tile_scene = scene_assets.deep_water.clone(); }
                    ShallowWater => { tile_scene = scene_assets.shallow_water.clone(); }
                    Sand => { tile_scene = scene_assets.sand.clone(); }
                    Grass => { tile_scene = scene_assets.grass.clone(); }
                    Street => { tile_scene = scene_assets.street.clone(); }
                    Hill => { tile_scene = scene_assets.hill.clone(); }
                    Mountain => { tile_scene = scene_assets.mountain.clone(); }
                    Snow => { tile_scene = scene_assets.snow.clone(); }
                    Lava => { tile_scene = scene_assets.lava.clone(); }
                    TileType::Teleport(_) => { tile_scene = scene_assets.teleport.clone(); }
                    Wall => { tile_scene = scene_assets.wall.clone();
                        tile_scale = Transform::from_scale(Vec3::new(0.5,1.5,0.5)).scale;}
                }
                match tile.content {
                    Rock(n) => {
                        if n < 2 {
                            content_scene = scene_assets.rock1.clone();
                            content_transform.scale = Transform::from_scale(Vec3::new(0.12,0.12,0.12)).scale;
                        }else if n < 3 {
                            content_scene = scene_assets.rock2.clone();
                            content_transform.scale = Transform::from_scale(Vec3::new(0.18,0.18,0.18)).scale;
                        }else {
                            content_scene = scene_assets.rock3.clone();
                            content_transform.scale = Transform::from_scale(Vec3::new(0.27,0.27,0.27)).scale;
                        }
                    }
                    Tree(n) => {
                        if n < 2 {
                            content_scene = scene_assets.tree1.clone();
                            content_transform.scale = Transform::from_scale(Vec3::new(0.09,0.09,0.09)).scale;
                        }else if n < 4 {
                            content_scene = scene_assets.tree2.clone();
                            content_transform.scale = Transform::from_scale(Vec3::new(0.08,0.08,0.08)).scale;
                        }else {
                            content_scene = scene_assets.tree3.clone();
                            content_transform.scale = Transform::from_scale(Vec3::new(0.12,0.12,0.12)).scale;
                        }
                    }
                    Garbage(_) => {
                        content_scene = scene_assets.garbage.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.005,0.005,0.005)).scale;
                    }
                    Fire => {
                        content_scene = scene_assets.fire.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.7,0.7,0.7)).scale;
                    }
                    Coin(_) => {
                        content_scene = scene_assets.coin.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(1.0,1.0,1.0)).scale;
                    }
                    Bin(_) => {
                        content_scene = scene_assets.bin.clone(); //TODO non mi piace troppo la resa
                        content_transform.scale = Transform::from_scale(Vec3::new(0.4,0.4,0.4)).scale;
                        content_transform.translation.y += 0.5;
                    }
                    Crate(_) => {
                        content_scene = scene_assets.crate_.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                    }
                    Bank(_) => {
                        content_scene = scene_assets.bank.clone(); //TODO non funziona
                        content_transform.scale = Transform::from_scale(Vec3::new(0.01,0.01,0.01)).scale;
                    }
                    Water(_) => {
                        content_scene = Default::default();
                    }
                    Market(_) => {
                        content_scene = scene_assets.market.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.4,0.4,0.4)).scale;
                    }
                    Fish(_) => {
                        content_scene = scene_assets.fish.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.02,0.02,0.02)).scale;
                    }
                    Building => {
                        content_scene = scene_assets.building.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.007,0.007,0.007)).scale;
                    }
                    Bush(_) => {
                        content_scene = scene_assets.bush.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.3,0.3,0.3)).scale;
                    }
                    JollyBlock(_) => {
                        content_scene = scene_assets.jolly_block.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                    }
                    Scarecrow => {
                        content_scene = scene_assets.scarecrow.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.3,0.3,0.3)).scale;
                    }
                    None => {
                        content_scene = Default::default();
                    }
                }
                use bevy::core_pipeline::{bloom::{BloomCompositeMode, BloomSettings}, tonemapping::Tonemapping,};
                commands.spawn(
                    (
                        TileBundle{
                            model: SceneBundle{
                                scene: tile_scene,
                                transform: Transform{
                                    translation: Transform::from_xyz(coordinates.0,(tile.elevation as f32 / 10.0) - 2.0 ,coordinates.1).translation,
                                    rotation: Default::default(),
                                    scale: tile_scale,
                                },
                                ..default()
                            },
                        },
                        TileComponent,
                    )
                );
                commands.spawn(
                    (
                        ContentBundle{
                            model: SceneBundle{
                                scene: content_scene,
                                transform: content_transform,
                                ..default()
                            }
                        },
                        ContentComponent
                    )
                );
                game_update.azioni.remove(0);
            }
            _ => {
                return;
            }
        }
    }
}
fn discover_tile(mut commands: Commands,
                 scene_assets: Res<SceneAssets>,
                 mut game_data: ResMut<GameData>,
                 mut game_update: ResMut<GameUpdate>,
){
    if game_update.azioni.len() != 0{
        match &game_update.azioni[0].0 {
            DiscoverTile{tile,coordinates,energy,points} => {
                let new_tile_radius = f32::sqrt((coordinates.0*coordinates.0) + (coordinates.1*coordinates.1));
                if new_tile_radius > game_data.map_radius {
                    game_data.map_radius = new_tile_radius;
                }
                game_data.robot_data.points += points;
                game_data.robot_data.points_update = *points;
                game_data.robot_data.energy += energy;
                game_data.robot_data.energy_update = *energy;
                let mut tile_scene;
                let mut tile_scale = Transform::from_scale(Vec3::new(0.5,0.5,0.5)).scale;
                let mut content_scene;
                let mut content_transform = Transform{
                    translation: Transform::from_xyz(coordinates.0,(tile.elevation as f32 / 10.0) - 2.0 ,coordinates.1).translation,
                    rotation: Default::default(),
                    scale: Transform::from_scale(Vec3::new(0.1,0.1,0.1)).scale,
                };
                match tile.tile_type {
                    DeepWater => { tile_scene = scene_assets.deep_water.clone(); }
                    ShallowWater => { tile_scene = scene_assets.shallow_water.clone(); }
                    Sand => { tile_scene = scene_assets.sand.clone(); }
                    Grass => { tile_scene = scene_assets.grass.clone(); }
                    Street => { tile_scene = scene_assets.street.clone(); }
                    Hill => { tile_scene = scene_assets.hill.clone(); }
                    Mountain => { tile_scene = scene_assets.mountain.clone(); }
                    Snow => { tile_scene = scene_assets.snow.clone(); }
                    Lava => { tile_scene = scene_assets.lava.clone(); }
                    TileType::Teleport(_) => { tile_scene = scene_assets.teleport.clone(); }
                    Wall => { tile_scene = scene_assets.wall.clone();
                        tile_scale = Transform::from_scale(Vec3::new(0.5,1.5,0.5)).scale;}
                }
                match tile.content {
                    Rock(n) => {
                        if n < 2 {
                            content_scene = scene_assets.rock1.clone();
                            content_transform.scale = Transform::from_scale(Vec3::new(0.12,0.12,0.12)).scale;
                        }else if n < 3 {
                            content_scene = scene_assets.rock2.clone();
                            content_transform.scale = Transform::from_scale(Vec3::new(0.18,0.18,0.18)).scale;
                        }else {
                            content_scene = scene_assets.rock3.clone();
                            content_transform.scale = Transform::from_scale(Vec3::new(0.27,0.27,0.27)).scale;
                        }
                    }
                    Tree(n) => {
                        if n < 2 {
                            content_scene = scene_assets.tree1.clone();
                            content_transform.scale = Transform::from_scale(Vec3::new(0.09,0.09,0.09)).scale;
                        }else if n < 4 {
                            content_scene = scene_assets.tree2.clone();
                            content_transform.scale = Transform::from_scale(Vec3::new(0.08,0.08,0.08)).scale;
                        }else {
                            content_scene = scene_assets.tree3.clone();
                            content_transform.scale = Transform::from_scale(Vec3::new(0.12,0.12,0.12)).scale;
                        }
                    }
                    Garbage(_) => {
                        content_scene = scene_assets.garbage.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.005,0.005,0.005)).scale;
                    }
                    Fire => {
                        content_scene = scene_assets.fire.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.7,0.7,0.7)).scale;
                    }
                    Coin(_) => {
                        content_scene = scene_assets.coin.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(1.0,1.0,1.0)).scale;
                    }
                    Bin(_) => {
                        content_scene = scene_assets.bin.clone(); //TODO non mi piace troppo la resa
                        content_transform.scale = Transform::from_scale(Vec3::new(0.4,0.4,0.4)).scale;
                        content_transform.translation.y += 0.5;
                    }
                    Crate(_) => {
                        content_scene = scene_assets.crate_.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                    }
                    Bank(_) => {
                        content_scene = scene_assets.bank.clone(); //TODO non funziona
                        content_transform.scale = Transform::from_scale(Vec3::new(0.0001,0.0001,0.0001)).scale;
                    }
                    Water(_) => {
                        content_scene = Default::default();
                    }
                    Market(_) => {
                        content_scene = scene_assets.market.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.4,0.4,0.4)).scale;
                    }
                    Fish(_) => {
                        content_scene = scene_assets.fish.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.02,0.02,0.02)).scale;
                    }
                    Building => {
                        content_scene = scene_assets.building.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.007,0.007,0.007)).scale;
                    }
                    Bush(_) => {
                        content_scene = scene_assets.bush.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.3,0.3,0.3)).scale;
                    }
                    JollyBlock(_) => {
                        content_scene = scene_assets.jolly_block.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                    }
                    Scarecrow => {
                        content_scene = scene_assets.scarecrow.clone();
                        content_transform.scale = Transform::from_scale(Vec3::new(0.3,0.3,0.3)).scale;
                    }
                    None => {
                        content_scene = Default::default();
                    }
                }
                commands.spawn(
                    (
                        TileBundle{
                            model: SceneBundle{
                                scene: tile_scene,
                                transform: Transform{
                                    translation: Transform::from_xyz(coordinates.0,(tile.elevation as f32 / 10.0) - 2.0 ,coordinates.1).translation,
                                    rotation: Default::default(),
                                    scale: tile_scale,
                                } ,
                                ..default()
                            },
                        },
                        TileComponent
                    )
                );
                commands.spawn(
                    (
                        ContentBundle{
                            model: SceneBundle{
                                scene: content_scene,
                                transform: content_transform,
                                ..default()
                            }
                        },
                        ContentComponent
                    )
                );
                game_data.next_action = true;
            }
            _ => {}
        }
    }
}
fn update_tile(mut tile_query: Query<(&Transform,&mut Handle<Scene>),With<TileComponent>>,
               scene_assets: Res<SceneAssets>,
               mut aggiornamento: ResMut<GameUpdate>,
               mut game_data: ResMut<GameData>,
){
    if !game_data.next_action{
        return;
    }else {
        if aggiornamento.azioni.len() != 0{
            match &aggiornamento.azioni[0].0 {
                UpdateTile{new_tile,back_pack_update,coordinates,energy,points} => {
                    for (transform, mut tile_scene) in tile_query.iter_mut(){
                        if transform.translation.x == coordinates.0 && transform.translation.z == coordinates.1 {
                            match new_tile.tile_type {
                                DeepWater => {
                                    *tile_scene = scene_assets.deep_water.clone();
                                }
                                ShallowWater => {
                                    *tile_scene = scene_assets.shallow_water.clone();
                                }
                                Sand => {
                                    *tile_scene = scene_assets.sand.clone();
                                }
                                Grass => {
                                    *tile_scene = scene_assets.grass.clone();
                                }
                                Street => {
                                    *tile_scene = scene_assets.street.clone();
                                }
                                Hill => {
                                    *tile_scene = scene_assets.hill.clone();
                                }
                                Mountain => {
                                    *tile_scene = scene_assets.mountain.clone();
                                }
                                Snow => {
                                    *tile_scene = scene_assets.snow.clone();
                                }
                                Lava => {
                                    *tile_scene = scene_assets.lava.clone();
                                }
                                TileType::Teleport(_) => {
                                    *tile_scene = scene_assets.teleport.clone();
                                }
                                Wall => {
                                    *tile_scene = scene_assets.wall.clone();
                                }
                            }
                        }
                    }
                }
                _ => {

                }
            }
        }
    }
}
fn update_content(mut content_query: Query<(&mut Transform,&mut Handle<Scene>),With<ContentComponent>>,
                  scene_assets: Res<SceneAssets>,
                  mut aggiornamento: ResMut<GameUpdate>,
                  mut game_data: ResMut<GameData>,
){
    if !game_data.next_action{
        return;
    }else {
        if aggiornamento.azioni.len() != 0{
            match &aggiornamento.azioni[0].0 {
                UpdateTile{new_tile,back_pack_update,coordinates,energy,points} => {
                    for i in back_pack_update.iter(){
                        let mut back_pack_value = game_data.robot_data.back_pack.get_mut(&i.0).unwrap();
                        *back_pack_value += i.1;
                    }
                    for i in back_pack_update.iter(){
                        let mut back_pack_value_delta = game_data.robot_data.back_pack_update.get_mut(&i.0).unwrap();
                        *back_pack_value_delta = i.1;
                    }
                    game_data.robot_data.points += points;
                    game_data.robot_data.points_update = *points;
                    game_data.robot_data.energy += energy;
                    game_data.robot_data.energy_update = *energy;
                    for (mut content_transform, mut content_scene) in content_query.iter_mut(){
                        //TODO risolvere per il building che ha l'offset
                        if content_transform.translation.x == coordinates.0 && content_transform.translation.z == coordinates.1{
                            match new_tile.content {
                                Rock(n) => {
                                    if n < 2 {
                                        *content_scene = scene_assets.rock1.clone();
                                        content_transform.scale = Transform::from_scale(Vec3::new(0.12,0.12,0.12)).scale;
                                    }else if n < 3 {
                                        *content_scene = scene_assets.rock2.clone();
                                        content_transform.scale = Transform::from_scale(Vec3::new(0.18,0.18,0.18)).scale;
                                    }else {
                                        *content_scene = scene_assets.rock3.clone();
                                        content_transform.scale = Transform::from_scale(Vec3::new(0.27,0.27,0.27)).scale;
                                    }
                                }
                                Tree(n) => {
                                    if n < 2 {
                                        *content_scene = scene_assets.tree1.clone();
                                        content_transform.scale = Transform::from_scale(Vec3::new(0.09,0.09,0.09)).scale;
                                    }else if n < 4 {
                                        *content_scene = scene_assets.tree2.clone();
                                        content_transform.scale = Transform::from_scale(Vec3::new(0.08,0.08,0.08)).scale;
                                    }else {
                                        *content_scene = scene_assets.tree3.clone();
                                        content_transform.scale = Transform::from_scale(Vec3::new(0.12,0.12,0.12)).scale;
                                    }
                                }
                                Garbage(_) => {
                                    *content_scene = scene_assets.garbage.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.005,0.005,0.005)).scale;
                                }
                                Fire => {
                                    *content_scene = scene_assets.fire.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.7,0.7,0.7)).scale;
                                }
                                Coin(_) => {
                                    *content_scene = scene_assets.coin.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(1.0,1.0,1.0)).scale;
                                }
                                Bin(_) => {
                                    *content_scene = scene_assets.bin.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.4,0.4,0.4)).scale;
                                    content_transform.translation.y += 0.5;
                                }
                                Crate(_) => {
                                    *content_scene = scene_assets.crate_.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                                }
                                Bank(_) => {
                                    *content_scene = scene_assets.bank.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.1,0.1,0.1)).scale;
                                }
                                Water(_) => {
                                    *content_scene = Default::default();
                                }
                                Market(_) => {
                                    *content_scene = scene_assets.market.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.4,0.4,0.4)).scale;
                                }
                                Fish(_) => {
                                    *content_scene = scene_assets.fish.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.02,0.02,0.02)).scale;
                                }
                                Building => {
                                    *content_scene = scene_assets.building.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.007,0.007,0.007)).scale;
                                }
                                Bush(_) => {
                                    *content_scene = scene_assets.bush.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.3,0.3,0.3)).scale;
                                }
                                JollyBlock(_) => {
                                    *content_scene = scene_assets.jolly_block.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.2,0.2,0.2)).scale;
                                }
                                Scarecrow => {
                                    *content_scene = scene_assets.scarecrow.clone();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.3,0.3,0.3)).scale;
                                }
                                None => {
                                    *content_scene = Default::default();
                                    content_transform.scale = Transform::from_scale(Vec3::new(0.1,0.1,0.1)).scale;
                                }
                            }
                        }
                    }
                }
                _ => {

                }
            }
        }
    }
}
fn hide_content_under_robot(mut content_query: Query<(&mut Transform, &mut Visibility),With<ContentComponent>>,
                            mut game_data: ResMut<GameData>,
){
    if game_data.content_visibility{
        if (f32::floor(game_data.robot_data.robot_translation.x) != game_data.hided_content.0) || (f32::floor(game_data.robot_data.robot_translation.z) != game_data.hided_content.1) {
            let mut new_hidden_content= (0.0,0.0);
            for (mut transform,mut visibility) in content_query.iter_mut(){
                if (f32::floor(transform.translation.x) == f32::floor(game_data.robot_data.robot_translation.x)) && (f32::floor(transform.translation.z) == f32::floor(game_data.robot_data.robot_translation.z)) {
                    new_hidden_content = (f32::floor(transform.translation.x),f32::floor(transform.translation.z));
                    *visibility = Visibility::Hidden;
                }else if (f32::floor(transform.translation.x) == game_data.hided_content.0) && (f32::floor(transform.translation.z) == game_data.hided_content.1) {
                    *visibility = Visibility::Visible;
                }
            }
            game_data.hided_content = new_hidden_content;
        }
    }
}
fn remove_game_update(mut game_update: ResMut<GameUpdate>,
                      mut game_data: ResMut<GameData>,
){
    if game_data.next_action{
        if game_update.azioni.len() > 0{
            game_data.feed.push(game_update.azioni[0].clone());
            game_update.azioni.remove(0);
        }
        game_data.next_action = false;
    }
}