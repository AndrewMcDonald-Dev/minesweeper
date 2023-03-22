use crate::events::TileMarkEvent;
use crate::events::TileTriggerEvent;
use crate::Board;
use bevy::input::mouse::MouseButtonInput;
use bevy::log;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn input_handling(
    window: Query<&Window, With<PrimaryWindow>>,
    board: Res<Board>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut tile_trigger_ewr: EventWriter<TileTriggerEvent>,
    mut tile_mark_ewr: EventWriter<TileMarkEvent>,
) {
    let window = window.get_single().unwrap();

    for event in button_evr.iter() {
        if event.state.is_pressed() {
            let position = window.cursor_position();
            if let Some(pos) = position {
                log::trace!("Mouse button pressed: {:?} as {}", event.button, pos);
                let tile_coordinates = board.mouse_position(window, pos);
                if let Some(coordinates) = tile_coordinates {
                    match event.button {
                        MouseButton::Left => {
                            log::info!("Trying to uncover tile on {}", coordinates);
                            tile_trigger_ewr.send(TileTriggerEvent(coordinates));
                        }
                        MouseButton::Right => {
                            log::info!("Trying to mark tile on {}", coordinates);
                            tile_mark_ewr.send(TileMarkEvent(coordinates));
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}
