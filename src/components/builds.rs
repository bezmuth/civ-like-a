use amethyst::ecs::prelude::{DenseVecStorage, Component};
#[derive(Copy, Clone, PartialEq)]
pub enum BuildingType{ // resource as it has no component implmentation, use READ and READEXPECT
    Center = 5,
    WarBuilding = 6,
    WoodBuilding = 7,
    MetalBuilding = 8,
    ScienceBuilding = 9,
    Demolish = 99,
}

pub struct Build{
    pub mode: Option<BuildingType>,
}

pub struct Building{ 
    pub buildingtype: BuildingType,
    pub playernum: i32,
    pub x: i32,
    pub y: i32,
}
impl Component for Building { // Component therefore use ReadStorage  an
    type Storage = DenseVecStorage<Self>;
}