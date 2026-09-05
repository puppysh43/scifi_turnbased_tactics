//gamestate
use egor::{input::Input, math::IVec2};
use hecs::World;
pub enum ControlState {
    Moving,
    Reticule(SelectingState),
}
pub enum SelectingState {
    Attacking,
}
pub struct GameState {
    world: World,
    ///the raw gamemap
    map: GameMap,
    ///the left and uppermost tile of the camera view
    camera_pos: IVec2,
    control_state: ControlState,
}
impl GameState {
    ///Creates completely blank gamestate
    pub fn new() -> GameState {
        GameState {
            world: World::new(),
            map: GameMap::new(),
            camera_pos: IVec2::new(0, 0),
        }
    }
    ///Once implemented will allow a custom level to be loaded from a file
    pub fn from_file() {}
    ///run systems
    pub fn run(&mut self, input: &Input) {
        //get input
    }
    ///draw game to screen
    pub fn draw() {
        //
    }
}
fn get_input(state: &mut GameState, input: &Input) {}
pub struct GameMap {
    ///height of the map
    h: i32,
    ///width of the map
    w: i32,
    map: Vec<TileType>,
}
impl GameMap {
    pub fn new() -> GameMap {
        GameMap {
            h: 64,
            w: 128,
            map: vec![TileType::Floor; 4 * 128],
        }
    }
    pub fn blank_with_size(h: i32, w: i32) -> GameMap {
        GameMap {
            h,
            w,
            map: vec![TileType::Floor; (h * w) as usize],
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    HalfCover,
    FullCover,
}
