//input function
use crate::components::*;
use crate::gamestate::events::*;
use crate::gamestate::{ControlState, GameState, SelectingState};
use egor::input::*;
use egor::math::IVec2;
use hecs::*;
/*reads the input from the frame context and adds messages of intent to the */
pub fn game_input(gamestate: &mut GameState, input: &mut &Input) {
    //
    match gamestate.control_state {
        ControlState::Root => {
            //in the root state the user can move the camera with the arrow keys and press enter or space
            //to bring up the selection reticule
            let delta = get_delta(input);
            if delta.is_some() {
                gamestate.camera_pos.saturating_add(delta.unwrap());
            }
            //spawn a selection reticule and set it to the appropriate control state.
            if input.keys_pressed(&[KeyCode::Enter, KeyCode::NumpadEnter, KeyCode::Space]) {
                gamestate
                    .world
                    .spawn((Reticule, Position::new(IVec2::new(0, 0))));
            }
        }
        //once the player has selected a unit they can decide what to do with that unit!
        ControlState::SelectedUnit => {
            //press m to move the unit
            if input.key_pressed(KeyCode::KeyM) {
                gamestate.control_state = ControlState::Moving;
            }
            //press a to attack
            if input.key_pressed(KeyCode::KeyA) {
                gamestate.control_state = ControlState::Reticule(SelectingState::Attacking);
            }
        }
        //if the player is moving a unit first identify the selected unit and then make the appropriate moi events
        ControlState::Moving => {
            //get the delta that was gathered from a keypress
            let delta = get_delta(input);
            if delta.is_some() {
                let mut mover: Option<Entity> = None;
                for (entity, _tag) in gamestate.world.query_mut::<(Entity, &Selected)>() {
                    mover = Some(entity.clone());
                }
                if mover.is_some() {
                    gamestate.events.push(Event::WantsMove(MoiMove::new(
                        mover.unwrap(),
                        delta.unwrap(),
                    )));
                }
            }
            //for right now let the player move a unit as much as they want and stop with the escape key
            if input.key_pressed(KeyCode::Escape) {
                //set the control state back to the menu for after a unit has been selected
                gamestate.control_state = ControlState::SelectedUnit;
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

///generic function for getting a delta from user keypresses
fn get_delta(input: &mut &Input) -> Option<IVec2> {
    if input.keys_pressed(&[KeyCode::ArrowLeft, KeyCode::Numpad4]) {
        //move the entity left
        Some(IVec2::new(-1, 0))
    } else if input.keys_pressed(&[KeyCode::ArrowUp, KeyCode::Numpad8]) {
        //move the entity up
        Some(IVec2::new(0, -1))
    } else if input.keys_pressed(&[KeyCode::ArrowRight, KeyCode::Numpad6]) {
        //move the entity right
        Some(IVec2::new(1, 0))
    } else if input.keys_pressed(&[KeyCode::ArrowDown, KeyCode::Numpad2]) {
        //move the entity down
        Some(IVec2::new(0, 1))
    } else if input.key_pressed(KeyCode::Numpad7) {
        //move the entity up and left diagonally
        Some(IVec2::new(-1, -1))
    } else if input.key_pressed(KeyCode::Numpad9) {
        //move the entity up and to the right diagonally
        Some(IVec2::new(1, -1))
    } else if input.key_pressed(KeyCode::Numpad3) {
        //move the entity down and to the right diagonally
        Some(IVec2::new(1, 1))
    } else if input.key_pressed(KeyCode::Numpad1) {
        //move the entity down and to the left diagonally
        Some(IVec2::new(-1, 1))
    } else {
        None
    }
}
