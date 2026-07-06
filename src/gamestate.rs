//gamestate
use egor::math::IVec2;
use hecs::World;
pub struct GameState {
    world: World,
    ///the raw gamemap
    map: GameMap,
    ///the left and uppermost tile of the camera view
    camera_pos: IVec2,
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
