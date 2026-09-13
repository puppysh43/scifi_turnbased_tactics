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
                //not entirely used to iterators and used this on others advice.
                let mover = gamestate
                    .world
                    .query_mut::<(Entity, &Selected)>()
                    .into_iter()
                    .last()
                    .map(|(entity, _)| entity.clone());
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
                    let delta = get_delta(input);
                    if delta.is_some() {
                        let mover = gamestate
                            .world
                            .query_mut::<(Entity, &Reticule)>()
                            .into_iter()
                            .last()
                            .map(|(entity, _)| entity.clone());
                        if mover.is_some() {
                            gamestate.events.push(Event::WantsMove(MoiMove::new(
                                mover.unwrap(),
                                delta.unwrap(),
                            )));
                        }
                    }
                    if input.keys_pressed(&[KeyCode::Enter, KeyCode::NumpadEnter]) {
                        //need to go over this one and chain it together more smoothly
                        //get the position of the reticule
                        let reticule_pos = get_reticule_pos(&mut gamestate.world);
                        //see if the position of the reticule matches the position of any other entities
                        let selected_entity =
                            check_entity_collision(reticule_pos, &mut gamestate.world);
                        //if so add the selected component to them

                        //then delete all reticules!
                    }
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

///reusable helper function for getting the position of the reticule, assuming only one exists
fn get_reticule_pos(world: &mut World) -> IVec2 {
    let pos = world
        .query_mut::<With<&IVec2, &Reticule>>()
        .into_iter()
        .last()
        .map(|reticule_pos| reticule_pos.clone());
    pos.expect("There is no reticule to get the position of!")
}
///checks if any entities are at the specified point and returns their identity id
fn check_entity_collision(pos: IVec2, world: &mut World) -> Option<Entity> {
    //use new form factor since we are only ever selecting one entity
    let mut colliding_entity: Option<Entity> = None;

    for (entity, entity_position) in world.query_mut::<(Entity, &Position)>() {
        if entity_position.get() == pos {
            colliding_entity = Some(entity.clone());
        }
    }
    colliding_entity
}
//deletes the reticule
