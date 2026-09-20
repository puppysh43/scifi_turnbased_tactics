//this will hold the draw implentation for the gamestate
use crate::gamestate::*;
use egor::render::Graphics;
//define this properly later
//maybe have the constants dynamically defined by the size of the window? idfk
///Constant that defines the amount of rows of tiles displayed in the camera's view (how much of a range in y)
const CAMERA_HEIGHT: i32 = 10;
///Constant that defines the amount of columns of tiles displayed in the camera's view (how much of a range in x values)
const CAMERA_WIDTH: i32 = 10;
//The width of individual tiles in pixels
const TILE_WIDTH: i32 = 16;
const TILE_HEIGHT: i32 = 16;
impl GameState {
    pub fn draw(&self, gfx: &mut Graphics) {
        //
    }
}
