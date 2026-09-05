//this will be where the game systems go I suppose
mod input;
use crate::{gamestate::GameState, gamesystems::input::game_input};
use egor::input::Input;
impl GameState {
    pub fn run(&mut self, input: &mut &Input) {
        //get input
        game_input(self, input);
    }
}
