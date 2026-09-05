//input function
use crate::gamestate::{ControlState, GameState, SelectingState};
use egor::input::*;
/*reads the input from the frame context and adds messages of intent to the */
pub fn game_input(gamestate: &mut GameState, input: &mut &Input) {
    //
    match gamestate.control_state {
        //if the player is moving a unit first identify the selected unit and then make the appropriate moi events
        ControlState::Moving => {
            if input.keys_pressed(&[KeyCode::ArrowLeft, KeyCode::Numpad4]) {
                //moi to move the unit left
            }
        }
        ControlState::Reticule(selecting_state) => {
            match selecting_state {
                SelectingState::SelectingUnit => {
                    //move the reticule with arrow keys or the numpad
                    //if enter is pressed on a unit then tag it with the selected component.
                }
                SelectingState::Attacking => {
                    //this will happen if a unit is selected
                }
            }
        }
    }
}
