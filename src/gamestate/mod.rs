//gamestate
mod draw;
mod events;
mod gamesystems;
use egor::{input::Input, math::IVec2};
use events::Event;
use hecs::World;

#[derive(Copy, Clone, Debug)]
pub enum ControlState {
    Root,
    SelectedUnit,
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
            control_state: ControlState::Root,
            events: Vec::new(),
        }
    }
    ///Once implemented will allow a custom level to be loaded from a file
    pub fn from_file() {}
}
pub struct GameMap {
    ///height of the map
    height: i32,
    ///width of the map
    width: i32,
    map: Vec<TileType>,
}
impl GameMap {
    pub fn new() -> GameMap {
        GameMap {
            height: 64,
            width: 128,
            map: vec![TileType::Floor; 64 * 128],
        }
    }
    pub fn blank_with_size(height: i32, width: i32) -> GameMap {
        GameMap {
            height,
            width,
            map: vec![TileType::Floor; (height * width) as usize],
        }
    }
    pub fn in_bounds(&self, point: IVec2) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }
    pub fn get_tile_from_point(&self, point: IVec2) -> &TileType {
        &self.map[self.get_index(point)]
    }
    pub fn get_index(&self, point: IVec2) -> usize {
        ((point.y * self.width) + point.x) as usize
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    HalfCover,
    FullCover,
}
