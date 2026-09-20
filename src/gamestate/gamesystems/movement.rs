//movement.rs
use super::utility::*;
use crate::gamestate::*;
pub fn system(gamestate: &mut GameState) {
    //iterate through the events queue in the gamestate
    //when it comes across a wants to move event, process it
    //this involves checking if the move would intersect with any other entities, if it would put the unit out of bounds
    //or if it would put the unit into a wall or other non traversable terrain
    //if it wouldn't do any of those things then move it!
    //
    //there is probably a better more concise way to do this but whatever
    gamestate.events.retain_mut(|event| {
        match event {
            Event::WantsMove(moi) => {
                //process the moi
                let (mover, delta) = moi.get();
                let potential_pos = get_pos(mover, &mut gamestate.world).saturating_add(delta);
                //check if potential pos is out of bounds of the map.
                // if
                //check if potential pos is outside of the camera frame and if so move the camera with the delta
                //check if the mover is a reticule, if it is then move the reticule b/c they don't need to worry about collission
                //keep going if the mover isn't a reticule - now we need to check that the tile the unit is moving into is a floor!
                //then last but not least check to make sure the unit is not moving into another unit! for now just check if there are any overlapping entities
                //BUT this will get more complicated in the future
                //if none of these issues arise you can overwrite the position of the mover and complete the move!

                false //this marks that we are not retaining it and "consuming" the event regardless of if the move is valid
            }
            //if it's any event other the one we're processing
            _ => {
                true //leave it alone for other systems!
            }
        }
    });
}
