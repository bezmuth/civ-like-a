use amethyst::ecs::{NullStorage, prelude::{DenseVecStorage, Component}};
use super::BuildingType;
pub struct Tiles{ 
    pub player: i32,
    pub buildingtype: Option<BuildingType>,
    pub x: i32,
    pub y: i32,
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