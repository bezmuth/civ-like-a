mod builds;
mod player;
mod tiles;

// * Some of these arent components they just fit the catagory
pub use self::{builds::{BuildingType, Build, Building}, player::Player, tiles::Tiles};