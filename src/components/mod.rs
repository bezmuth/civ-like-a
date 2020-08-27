mod builds;
mod player;
mod tiles;
mod resbar;

// * Some of these arent components they just fit the catagory
pub use self::{builds::{BuildingType, Build, Building}, player::{Player, PlayersInfo}, tiles::{Tiles, Layer1, Layer2, Layer3}, resbar::Resbar};