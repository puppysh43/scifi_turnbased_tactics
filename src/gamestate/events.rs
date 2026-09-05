//events enum and related info

use egor::math::IVec2;
use hecs::Entity;

pub enum Event {
    WantsMove(MoiMove),
}

pub struct MoiAttack {
    attacker: Entity,
    defender: Entity,
}
impl MoiAttack {
    pub fn new(attacker: Entity, defender: Entity) -> MoiAttack {
        MoiAttack { attacker, defender }
    }
    ///Returns the attacker and defender in that order
    pub fn get(self) -> (Entity, Entity) {
        (self.attacker, self.defender)
    }
}
pub struct MoiMove {
    mover: Entity,
    delta: IVec2,
}
impl MoiMove {
    pub fn new(mover: Entity, delta: IVec2) -> MoiMove {
        MoiMove { mover, delta }
    }
    ///Returns the mover and delta in that order
    pub fn get(&self) -> (Entity, IVec2) {
        (self.mover, self.delta)
    }
}
