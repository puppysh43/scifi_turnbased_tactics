//movement.rs
use super::utility::*;
use crate::components::*;
use crate::gamestate::draw::{CAMERA_HEIGHT, CAMERA_WIDTH};
use crate::gamestate::*;
use hecs::*;
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
                if gamestate.map.in_bounds(potential_pos) {
                    //check if potential pos is outside of the camera frame and if so move the camera with the delta
                    if !in_camera_view(gamestate.camera_pos, potential_pos) {
                        gamestate.camera_pos.saturating_add(delta);
                    }
                    //check if the mover is a reticule, if it is then move the reticule b/c they don't need to worry about collission
                    if is_reticule(mover, &mut gamestate.world) {
                        //fulfill the request to move and update the position of the entity!
                        move_entity(mover, &mut gamestate.world, potential_pos);
                    } else {
                        //however if it is not a reticule then you need to check for collission with walls and other entities
                        // check that there are no collisions with entities or map tiles
                        if !check_map_collision(potential_pos, &gamestate.map)
                            && !check_entity_collision(potential_pos, &mut gamestate.world)
                        {
                            move_entity(mover, &mut gamestate.world, potential_pos);
                        }
                    }
                }
                false //this marks that we are not retaining it and "consuming" the event regardless of if the move is valid
            }
            //if it's any event other the one we're processing
            _ => {
                true //leave it alone for other systems!
            }
        }
    });
}
//in the future maybe collision should have just a binary component for "This can collide" but not relevant right now

fn in_camera_view(camera_pos: IVec2, entity_pos: IVec2) -> bool {
    entity_pos.x >= camera_pos.x
        && entity_pos.x < (camera_pos.x + CAMERA_WIDTH)
        && entity_pos.y >= camera_pos.y
        && entity_pos.y < (camera_pos.y + CAMERA_HEIGHT)
}

fn is_reticule(entity: Entity, world: &mut World) -> bool {
    world.query_one_mut::<&Reticule>(entity).is_ok()
}

fn move_entity(mover: Entity, world: &mut World, destination: IVec2) {
    let current_position = world
        .query_one_mut::<&mut Position>(mover)
        .expect("Entity being moved doesn't have a position component!");
    current_position.set(destination);
    println!(
        "An entity has moved to x: {} y: {}",
        destination.x, destination.y
    );
}

///checks if the potential new position is a non-enterable tile. returns true if there is a collission and false if there isn't
fn check_map_collision(potential_pos: IVec2, map: &GameMap) -> bool {
    map.get_tile_from_point(potential_pos) != &TileType::Floor
}

///checks if the potential new position overlaps with another entity's position (that is not a reticule). returns true if there is a collission
fn check_entity_collision(potential_pos: IVec2, world: &mut World) -> bool {
    let mut collision = false;
    for entity_pos in world.query_mut::<Without<&Position, &Reticule>>() {
        if entity_pos.get() == potential_pos {
            collision = true;
        }
    }
    collision
}
