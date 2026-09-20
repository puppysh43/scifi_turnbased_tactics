//this will be where the game systems go I suppose
mod attacking;
mod input;
mod movement;
mod utility;
use crate::gamestate::GameState;
use egor::input::Input;
impl GameState {
    pub fn update(&mut self, input: &mut &Input) {
        //get input
        input::system(self, input);
        //then process moves
        movement::system(self);
        //then process attacks
        attacking::system(self);
    }
}
