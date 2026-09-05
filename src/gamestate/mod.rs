//gamestate
mod events;
mod gamesystems;
use egor::{input::Input, math::IVec2};
use events::Event;
use hecs::World;

#[derive(Copy, Clone, Debug)]
pub enum ControlState {
    Moving,
    Reticule(SelectingState),
}

#[derive(Copy, Clone, Debug)]
pub enum SelectingState {
    SelectingUnit,
    Attacking,
}
pub struct GameState {
    world: World,
    ///the raw gamemap
    map: GameMap,
    ///the left and uppermost tile of the camera view
    camera_pos: IVec2,
    control_state: ControlState,
    events: Vec<Event>,
}
impl GameState {
    ///Creates completely blank gamestate
    pub fn new() -> GameState {
        GameState {
            world: World::new(),
            map: GameMap::new(),
            camera_pos: IVec2::new(0, 0),
            control_state: ControlState::Reticule(SelectingState::SelectingUnit),
            events: Vec::new(),
        }
    }
    ///Once implemented will allow a custom level to be loaded from a file
    pub fn from_file() {}
    ///run systems
    ///draw game to screen
    pub fn draw() {
        //
    }
}
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
