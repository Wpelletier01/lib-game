
use crate::shape::Rect;


pub struct TileInfo {

    id:     i16,
    src:    String,
    wall:   bool

}

impl TileInfo {

    pub fn new(id:i16,src:&str,wall:bool) -> Self { Self { id,src: src.to_string(), wall} }
    pub fn get_id(&self) -> i16 { self.id }
    pub fn get_src(&self) -> &str { &self.src }
    pub fn is_a_wall(&self) -> bool { self.wall }

}



