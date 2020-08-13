use amethyst::ecs::{Entity, prelude::{DenseVecStorage, Component}};
pub struct Resbar{
    pub top: Entity,
}

impl Component for Resbar { // Component therefore use ReadStorage  an
    type Storage = DenseVecStorage<Self>;
}