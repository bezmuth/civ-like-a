use amethyst::ecs::{NullStorage, prelude::{DenseVecStorage, Component}};

#[derive(PartialEq, Eq, Copy, Clone)] // Partial Eq and Eq for comparisons
                                      // Copy and Clone are used when I want to give 2 tiles on different layers the same pos
pub struct TilePos{
    pub x : i32,
    pub y: i32,
}

impl Component for TilePos{
    type Storage = DenseVecStorage<Self>;
}


#[derive(Copy, Clone, PartialEq)]
pub enum TileType{ // resource as it has no component implmentation, use READ and READEXPECT
    Empty = 4,
    Center = 5,
    Barrack = 6,
    WoodBuilding = 100, // not implemented
    MetalBuilding = 101, // not implemented
    ScienceBuilding = 102, // not implemented
    Location = 7,
    Demolish = 99,
}

pub struct Tiles{ 
    pub player: i32,
    pub TileType: Option<TileType>,
}
impl Component for Tiles { // Component therefore use ReadStorage  an
    type Storage = DenseVecStorage<Self>;
}

// * tag components
#[derive(Default)]
pub struct Layer1;
impl Component for Layer1 {
    type Storage = NullStorage<Self>;
}
#[derive(Default)]
pub struct Layer2;
impl Component for Layer2 {
    type Storage = NullStorage<Self>;
}
#[derive(Default)]
pub struct Layer3;
impl Component for Layer3 {
    type Storage = NullStorage<Self>;
}