use crate::vector::Vec2;

use std::f32::consts::PI;


pub enum ShapeType {

    Rect,
    Circle,
    Polygon

}


pub trait Shape {

    fn mov(&mut self,direction:Vec2);
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;

}

#[derive(Copy, Clone, Debug)]
pub struct Rect {

    pub pos:    Vec2,
    pub size:   Vec2

}

impl Rect {

    pub fn new(x:f32,y:f32,w:f32,h:f32) -> Self {
        Self { pos: Vec2::new(x,y) ,size: Vec2::new(w,h) } }

    pub fn get_pos(&self) -> Vec2      { self.pos   }
    pub fn get_size(&self) -> &Vec2     { &self.size  }
    pub fn get_width(&self) -> f32      { self.size.x }
    pub fn get_height(&self) -> f32     { self.size.y }

    pub fn get_max_x(&self) -> f32      { self.pos.x + self.size.x }
    pub fn get_max_y(&self) -> f32      { self.pos.y + self.size.y }



}

impl Shape for Rect {

    fn mov(&mut self, direction: Vec2) { self.pos += direction; }
    fn get_x(&self) -> f32 { self.pos.x }
    fn get_y(&self) -> f32 { self.pos.y }


}


pub struct Circle {

    pos:    Vec2,
    radius: f32

}

impl Circle {

    pub fn new(x:f32,y:f32,r:f32) -> Self { Self { pos: Vec2::new(x,y), radius: r }}
    pub fn get_pos(&self) -> Vec2 { self.pos }
    pub fn get_radius(&self) -> f32 { self.radius }
    pub fn get_x(&self) -> f32 { self.pos.x }
    pub fn get_y(&self) -> f32 { self.pos.y }

}

impl Shape for Circle {
    fn mov(&mut self, direction: Vec2) { self.pos += direction; }
    fn get_x(&self) -> f32 { self.pos.x }
    fn get_y(&self) -> f32 { self.pos.y }

}


pub struct Polygon {

    pos:        Vec2,
    nb_side:    u8,
    radius:     f32,
    rotation:   f32

}

impl Polygon {

    pub fn new(x:f32,y:f32,nb_side:u8,radius:f32,rotation:f32) -> Self {
        Self {
            pos: Vec2::new(x,y),
            nb_side,
            radius,
            rotation

        }
    }

    pub fn get_pos(&self) -> Vec2 { self.pos }
    pub fn get_nb_side(&self) -> u8 { self.nb_side }
    pub fn get_radius(&self) -> f32 { self.radius }
    pub fn get_rotation(&self) -> f32 { self.rotation }

    pub fn get_edges_position(&self) -> Vec<Vec2> {

        let mut edges:Vec<Vec2> = Vec::new();

        let central_angle = 360.0 / self.nb_side as f32;

        for side in 0..self.nb_side {

            let angle = radian(side as f32 * central_angle);

            // without the rotation apply
            let x = (self.pos.x + self.radius) * angle.cos();
            let y = (self.pos.y + self.radius) * angle.sin();

            let (rotated_x,rotated_y) = self.calcul_edge_rotated(x,y);

            edges.push(Vec2::new(rotated_x,rotated_y));

        }


        edges

    }

    fn calcul_edge_rotated(&self,x:f32,y:f32) -> (f32,f32){

        let rotation_radian = radian(self.rotation);

        let origin_x = x - self.pos.x;
        let origin_y = y - self.pos.y;

        let rotated_x =
            origin_x * rotation_radian.cos() - origin_y * rotation_radian.sin() + self.pos.x;
        let rotated_y = origin_x * rotation_radian.sin() + origin_y + rotation_radian.cos() + self.pos.y;

       (rotated_x,rotated_y)

    }


}


fn radian(value:f32) -> f32 {  value * (PI/180.0)  }