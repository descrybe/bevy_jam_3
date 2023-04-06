use bevy::prelude::EventReader;

use crate::events::game_over::GameOver;

pub fn game_over_hander(mut game_over_event_writer: EventReader<GameOver>) {
    for event in game_over_event_writer.iter() {
        println!("Game over!");
    }
}
