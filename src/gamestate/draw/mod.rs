//this will hold the draw implentation for the gamestate
use crate::components::*;
use crate::gamestate::*;
use egor::render::Graphics;
//define this properly later
//maybe have the constants dynamically defined by the size of the window? idfk
///Constant that defines the amount of rows of tiles displayed in the camera's view (how much of a range in y)
pub const CAMERA_HEIGHT: i32 = 10;
///Constant that defines the amount of columns of tiles displayed in the camera's view (how much of a range in x values)
pub const CAMERA_WIDTH: i32 = 10;
//The width of individual tiles in pixels
const TILE_WIDTH: i32 = 24;
const TILE_HEIGHT: i32 = 24;
//fuck it just have it be a fixed map for now who cares
impl GameState {
    pub fn draw(&self, gfx: &mut Graphics) {
        //first render the map
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                let point = IVec2::new(x, y);
                let index = self.map.get_index(point);
                if self.map.in_bounds(point) {
                    match self.map.tiles[index] {
                        TileType::Wall => {
                            //print the wall tile
                        }
                        TileType::Floor => {
                            //print the floor
                        }
                        TileType::HalfCover => {
                            //print half cover w/ a floor underneath it as well
                        }
                        TileType::FullCover => {
                            //print full cover w/ a floor underneath it
                        }
                    }
                }
            }
        }
        //then render out the entities
        for (sprite, position) in self.world.query_mut::<(&Sprite, &Position)>() {
            // print_tile
        }
    }
}

// fn print_tile()
/*    fn print_tile(&self, gfx: &mut Graphics, x: i32, y: i32, texture: &str) {
        gfx.rect()
            .size(Vec2::new(32.00, 32.00))
            .at(Vec2::new((x * TILE_WIDTH) as f32, (y * TILE_HEIGHT) as f32))
            .color(Color::WHITE)
            .texture(self.get_texture(texture));
    }
*/
