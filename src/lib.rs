#[macro_use]
extern crate simple_error;

pub mod vector;
pub mod shape;
pub mod collision;
pub mod loader;
pub mod tile;
pub mod sprite;


use std::error::Error;

pub type GResult<T> = Result<T,Box<dyn Error>>;


#[derive(PartialEq,Copy, Clone)]
pub enum Direction {
    Bottom,
    Top,
    Left,
    Right
}
