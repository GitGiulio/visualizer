use bevy::prelude::*;
use robotics_lib::world::tile::Content;
use crate::game_data::{GameData, MySet};
use crate::RobotAction::*;


#[derive(Component)]
pub struct EnergyComponent;
#[derive(Component)]
pub struct EnergyUpdateComponent;
#[derive(Component)]
pub struct PointsComponent;
#[derive(Component)]
pub struct PointsUpdateComponent;
#[derive(Component)]
pub struct BackPackComponent;
#[derive(Component)]
pub struct BackPackUpdateComponent;
#[derive(Component)]
pub struct FeedComponent;
pub struct GUIPlugin;

impl Plugin for GUIPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup,create_gui)
            .add_systems(Update,update_energy.in_set(MySet::Third))
            .add_systems(Update,update_energy_update.in_set(MySet::Third))
            .add_systems(Update,update_points.in_set(MySet::Third))
            .add_systems(Update,update_points_update.in_set(MySet::Third))
            .add_systems(Update,update_feed.in_set(MySet::Third))
            .add_systems(Update,update_backpack.in_set(MySet::Third))
            .add_systems(Update,update_backpack_update.in_set(MySet::Third));
    }
}
fn create_gui(mut commands: Commands,game_data: Res<GameData>){
    let energy_string = format!("Energy: {}",game_data.robot_data.energy);
    commands.spawn((
        TextBundle::from_section(
            energy_string,
            TextStyle {
                font_size: 30.0,
                color: Color::rgb(1.0, 1.0, 0.0),
                ..default()
            },
        ).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),EnergyComponent));
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 20.0,
                color: Color::rgb(1.0, 1.0, 0.0),
                ..default()
            },
        ).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(30.0),
            left: Val::Px(130.0),
            ..default()
        }),EnergyUpdateComponent));
    commands.spawn((
        TextBundle::from_section(
            "Points: 0",
            TextStyle {
                font_size: 30.0,
                color: Color::rgb(0.5, 0.1, 0.5),
                ..default()
            },
        ).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),PointsComponent));
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 20.0,
                color: Color::rgb(0.5, 0.1, 0.5),
                ..default()
            },
        ).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(30.0),
            left: Val::Px(130.0),
            ..default()
        }),PointsUpdateComponent));
    commands.spawn((
        TextBundle::from_section(
            "----BackPack----\n Water:0\n Rock:0\n Tree:0\n Bush:0\n JollyBlock:0   \n Garbage:0\n Coin:0\n Fish:0\n----BackPack----",
            TextStyle {
                font_size: 20.0,
                color: Color::rgb(0.0, 0.0, 0.0),
                ..default()
            },
        ).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),BackPackComponent));
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 20.0,
                color: Color::rgb(0.0, 0.0, 0.0),
                ..default()
            },
        ).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),BackPackUpdateComponent));
    commands.spawn((
        TextBundle::from_section(
            "-----Feed-----\n-----Feed-----",
            TextStyle {
                font_size: 20.0,
                color: Color::rgb(1.0, 1.0, 1.0),
                ..default()
            },
        ).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),FeedComponent));
}
fn update_energy(mut energy_query: Query<&mut Text,With<EnergyComponent>>,
                 mut game_data: ResMut<GameData>,
){
    let mut energy_text = energy_query.single_mut();
    energy_text.sections[0].value = format!("Energy: {}", game_data.robot_data.energy);
}
fn update_energy_update(mut energy_update_query: Query<&mut Text,With<EnergyUpdateComponent>>,
                        mut game_data: ResMut<GameData>,
                        time: Res<Time>,
){
    let mut energy_update_text = energy_update_query.single_mut();
    if game_data.robot_data.energy_update != 0{
        energy_update_text.sections[0].style.color = Color::rgba(1.0, 1.0, 0.0,1.0);
        if game_data.robot_data.energy_update > 0{
            energy_update_text.sections[0].value = format!("+{}", game_data.robot_data.energy_update);
        }else {
            energy_update_text.sections[0].value = format!("{}", game_data.robot_data.energy_update);
        }
        game_data.robot_data.energy_update = 0;
    }else {
        energy_update_text.sections[0].style.color = Color::rgba(1.0, 1.0, 0.0, energy_update_text.sections[0].style.color.a() - (0.8 * time.delta_seconds()));
    }
}
fn update_points(mut points_query: Query<&mut Text,With<PointsComponent>>,
                 mut game_data: ResMut<GameData>,
){
    let mut points_text = points_query.single_mut();
    points_text.sections[0].value = format!("Points: {}", game_data.robot_data.points);
}
fn update_points_update(mut points_update_query: Query<&mut Text,With<PointsUpdateComponent>>,
                        mut game_data: ResMut<GameData>,
                        time: Res<Time>,
){
    let mut points_update_text = points_update_query.single_mut();
    if game_data.robot_data.points_update != 0.0{
        points_update_text.sections[0].style.color = Color::rgba(0.5, 0.1, 0.5,1.0);
        if game_data.robot_data.points_update > 0.0{
            points_update_text.sections[0].value = format!("+{}", game_data.robot_data.points_update);
        }else {
            points_update_text.sections[0].value = format!("-{}", game_data.robot_data.points_update);
        }
        game_data.robot_data.points_update = 0.0;
    }else {
        points_update_text.sections[0].style.color = Color::rgba(0.5, 0.1, 0.5, points_update_text.sections[0].style.color.a() - (0.8 * time.delta_seconds()));
    }
}
fn update_feed(mut feed_query: Query<&mut Text,With<FeedComponent>>,
                   mut game_data: ResMut<GameData>,
) {
    let mut feed_text = feed_query.single_mut();
    if !game_data.feed_visibility{
        feed_text.sections[0].value = format!("");
    }else {
        let mut feed_string = String::new();
        let mut i = game_data.feed.len() as i32 - 7;
        while i < game_data.feed.len() as i32{
            if i >= 0 {
                feed_string.push_str("\n");
                let tmp = format!("{:?}",game_data.feed[i as usize].0);
                feed_string.push_str(&tmp);
            }
            i += 1;
        }
        feed_text.sections[0].value = format!("-----Feed-----{}\n-----Feed-----", feed_string);
    }
}
fn update_backpack(mut back_pack_query: Query<&mut Text,With<BackPackComponent>>,
                   mut game_data: ResMut<GameData>,
) {
    let mut back_pack_text = back_pack_query.single_mut();
    if !game_data.robot_data.back_pack_visibility{
        back_pack_text.sections[0].value = format!("");
    }else {
        let water = game_data.robot_data.back_pack.get(&Content::Water(0)).unwrap();
        let tree = game_data.robot_data.back_pack.get(&Content::Tree(0)).unwrap();
        let rock = game_data.robot_data.back_pack.get(&Content::Rock(0)).unwrap();
        let fish = game_data.robot_data.back_pack.get(&Content::Fish(0)).unwrap();
        let jolly_block = game_data.robot_data.back_pack.get(&Content::JollyBlock(0)).unwrap();
        let bush = game_data.robot_data.back_pack.get(&Content::Bush(0)).unwrap();
        let garbage = game_data.robot_data.back_pack.get(&Content::Garbage(0)).unwrap();
        let coin = game_data.robot_data.back_pack.get(&Content::Coin(0)).unwrap();
        back_pack_text.sections[0].value = format!("----BackPack----\n Water:{}\n Rock:{}\n Tree:{}\n Bush:{}\n JollyBlock:{}   \n Garbage:{}\n Coin:{}\n Fish:{}\n----BackPack----", water, rock, tree, bush, jolly_block, garbage, coin, fish);
    }
}
fn update_backpack_update(mut back_pack_update_query: Query<&mut Text,With<BackPackUpdateComponent>>,
                          mut game_data: ResMut<GameData>,
                          time: Res<Time>,
) {
    let mut back_pack_update_text = back_pack_update_query.single_mut();
    if !game_data.robot_data.back_pack_visibility{
        back_pack_update_text.sections[0].value = format!("");
    }else {
        let mut str = String::from("");
        let mut v = vec![];
        v.push(game_data.robot_data.back_pack_update.get(&Content::Water(0)).unwrap());
        v.push(game_data.robot_data.back_pack_update.get(&Content::Rock(0)).unwrap());
        v.push(game_data.robot_data.back_pack_update.get(&Content::Tree(0)).unwrap());
        v.push(game_data.robot_data.back_pack_update.get(&Content::Bush(0)).unwrap());
        v.push(game_data.robot_data.back_pack_update.get(&Content::JollyBlock(0)).unwrap());
        v.push(game_data.robot_data.back_pack_update.get(&Content::Garbage(0)).unwrap());
        v.push(game_data.robot_data.back_pack_update.get(&Content::Coin(0)).unwrap());
        v.push(game_data.robot_data.back_pack_update.get(&Content::Fish(0)).unwrap());

        let mut update = false;

        for i in v {
            if *i > 0{
                str.push_str(format!("\n+{}", i).as_str());
                update = true;
            }else if *i < 0 {
                str.push_str(format!("\n{}",i).as_str());
                update = true;
            }else {
                str.push_str("\n");
            }
        }
        game_data.robot_data.back_pack_update.insert(Content::Water(0), 0);
        game_data.robot_data.back_pack_update.insert(Content::Tree(0), 0);
        game_data.robot_data.back_pack_update.insert(Content::Rock(0), 0);
        game_data.robot_data.back_pack_update.insert(Content::Fish(0), 0);
        game_data.robot_data.back_pack_update.insert(Content::JollyBlock(0), 0);
        game_data.robot_data.back_pack_update.insert(Content::Bush(0), 0);
        game_data.robot_data.back_pack_update.insert(Content::Garbage(0), 0);
        game_data.robot_data.back_pack_update.insert(Content::Coin(0), 0);

        if update {
            back_pack_update_text.sections[0].value = str;
            back_pack_update_text.sections[0].style.color = Color::rgba(0.0, 0.0, 0.0,1.0);
        }else{
            back_pack_update_text.sections[0].style.color = Color::rgba(0.0, 0.0, 0.0, back_pack_update_text.sections[0].style.color.a() - (0.8 * time.delta_seconds()));
        }
    }
}

/*
fn show_info(query: Query<Info,With<GUI>> ,keyboard_input: Res<Input<KeyCode>>, mut game_data: ResMut<GameData>){
    //TODO (quando I sta venendo premuto faccio vedere le info oscurando il resto)
}*/