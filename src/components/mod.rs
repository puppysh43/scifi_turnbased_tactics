use egor::math::IVec2;
/*
file with all the components necessary for the game
*/
use hecs::Entity;

pub struct Health {
    max: i32,
    current: i32,
}
impl Health {
    pub fn new(max: i32) -> Health {
        Health { max, current: max }
    }
    pub fn hurt(&mut self, dmg: i32) {
        self.current -= dmg;
    }
    pub fn heal(&mut self, heal: i32) {
        self.current += heal;
        if self.current >= self.max {
            self.current = self.max;
        }
    }
}
pub struct Weapon {
    dmg: i32,
}
impl Weapon {
    pub fn new(dmg: i32) -> Weapon {
        Weapon { dmg }
    }
    pub fn get_dmg(&self) -> i32 {
        self.dmg
    }
}

pub struct Position(IVec2);
impl Position {
    pub fn new(pos: IVec2) -> Position {
        Position(pos)
    }
    pub fn get(&self) -> IVec2 {
        self.0
    }
    pub fn set(&mut self, new_pos: IVec2) {
        self.0 = new_pos;
    }
    pub fn add_delta(&mut self, delta: IVec2) {
        self.0.saturating_add(delta);
    }
}
