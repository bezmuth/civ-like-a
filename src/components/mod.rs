mod builds;
mod player;
mod tiles;
mod resbar;
mod unit;
mod stats;

// * Some of these arent components they just fit the catagory
pub use self::{builds::{Build, Building, OutPos}, player::{Player, PlayersInfo}, tiles::{Tiles, TilePos, TileType, Layer1, Layer2, Layer3}, resbar::Resbar, unit::{Unit, UnitStack}, stats::Stat};
