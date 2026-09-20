//this will hold helper functions that are needed between multiple systems
use crate::components::*;
use egor::math::IVec2;
use hecs::*;
//get the position as a raw IVec2 of any entity. only use if you're sure the entity has a position!

pub fn get_pos(entity: Entity, world: &mut World) -> IVec2 {
    let pos = world.query_one_mut::<&Position>(entity).unwrap().get();
    pos
}
