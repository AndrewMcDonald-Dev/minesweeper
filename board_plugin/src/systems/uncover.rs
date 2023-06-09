use crate::{
    components::Uncover,
    events::{BoardCompletedEvent, BombExplosionEvent},
    Board, Bomb, BombNeighbor, Coordinates,
};
use bevy::{log, prelude::*};

use crate::events::TileTriggerEvent;

pub fn trigger_event_handler(
    mut commands: Commands,
    board: Res<Board>,
    mut tile_trigger_evr: EventReader<TileTriggerEvent>,
) {
    for trigger_event in tile_trigger_evr.iter() {
        if let Some(entity) = board.tile_to_uncover(&trigger_event.0) {
            commands.entity(*entity).insert(Uncover);
        }
    }
}

pub fn uncover_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    children: Query<(Entity, &Parent), With<Uncover>>,
    parents: Query<(&Coordinates, Option<&Bomb>, Option<&BombNeighbor>)>,
    mut board_completed_event_wr: EventWriter<BoardCompletedEvent>,
    mut bomb_explosion_event_wr: EventWriter<BombExplosionEvent>,
) {
    //Iterate through tiles covers to uncover
    // ? May be more efficint to store covers in a hashmap to reduce comparisons
    for (entity, parent) in children.iter() {
        //we destroy the tile cover entity
        commands.entity(entity).despawn_recursive();
        let (coords, bomb, bomb_counter) = match parents.get(parent.get()) {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}", e);
                continue;
            }
        };

        // Remove the entity from the board covered tile map
        match board.try_uncover_tile(coords) {
            None => log::debug!("Tried to uncover an already uncovered tile."),
            Some(e) => log::debug!("Uncovered tile {} (entity:{:?})", coords, e),
        }

        if board.is_completed() {
            log::info!("Board completed.");
            board_completed_event_wr.send(BoardCompletedEvent);
        }
        if bomb.is_some() {
            log::info!("Boom!!");
            bomb_explosion_event_wr.send(BombExplosionEvent);
        } else if bomb_counter.is_none() {
            // Propagate the uncovering of adjacent tiles which will be removed next frame
            for entity in board.adjacent_covered_tiles(*coords) {
                commands.entity(entity).insert(Uncover);
            }
        }
    }
}
