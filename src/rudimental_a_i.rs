use bevy::prelude::*;
use crate::game_data::*;
use crate::{AI, GameUpdate, TestAI};

pub struct ArtificialIntelligencePlugin;

impl Plugin for ArtificialIntelligencePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_game_update.in_set(MySet::First));
    }
}
fn update_game_update(mut game_update: ResMut<GameUpdate>,
                             mut game_data: ResMut<GameData>,
                             time: Res<Time>,
){
    if game_data.next!=0{
        game_data.next -= 1;
        info!("next process_tick");
        /*let update = artificial_intelligence.next();
        for i in update{
            game_update.azioni.push(i);
        }*/
    }
}
