use hecs::*;

//attacking
use crate::{components::Health, gamestate::*};
pub fn system(gamestate: &mut GameState) {
    //minimum to get this working is literally just to not check for anything and decrement the health of the attacked entity and then delete an entity if the health goes to 0
    gamestate.events.retain_mut(|event| {
        match event {
            Event::WantsAttack(moi) => {
                //decrement defending entity health by 1
                let (_attacker, defender) = moi.get();
                let defender_health = gamestate
                    .world
                    .query_one_mut::<&mut Health>(defender)
                    .expect("Attacked entity doesn't have a health component!");
                defender_health.hurt(1);
                println!(
                    "Attacked entity currently has a health of {}",
                    defender_health.get()
                );
                //then check for any entities with 0 health and remove them
                let mut command_buffer = CommandBuffer::new();
                for (entity_id, health) in gamestate.world.query_mut::<(Entity, &Health)>() {
                    if health.get() <= 0 {
                        command_buffer.despawn(entity_id);
                    }
                }
                command_buffer.run_on(&mut gamestate.world);
                false
            }
            _ => {
                //do nothing with the other moi's and move on
                true
            }
        }
    });
}
