use super::TilePos;

#[derive(Copy, Clone, PartialEq)]
pub enum PathObjType{ // resource as it has no component implmentation, use READ and READEXPECT
    Empty,
    Start,
    Target,
    Obstacle,
}
impl Default for PathObjType {
    fn default() -> Self{PathObjType::Empty}
}


#[derive(Default, Copy, Clone)]
pub struct PathTileInfo {
    pub tile_type : PathObjType,
    pub fcost : i32,
    pub closed: bool,
    pub path: bool
}


pub struct Path{ // resource that stores the current path
    pub path_arr: [[PathTileInfo; 40]; 40], // the path will only ever get so large, theres no need to have an array larger than 40x40
    pub start_pos: TilePos, // location of the start of the path in terms of the tilemap
    pub end_pos: TilePos, // location of the start of the path in terms of the tilemap
    pub start: bool,
    pub complete: bool,
}
