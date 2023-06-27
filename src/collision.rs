
use crate::shape::{Circle,Rect,Polygon,Shape};
use crate::vector::Vec2;



pub fn circle_vs_circle(c1:&Circle,c2:&Circle) -> bool {

    let mut a = (c1.get_x() - c2.get_x()).abs();
    let mut b = (c1.get_y() - c2.get_y()).abs();

    // square
    a = a.powf(2.0);
    b = b.powf(2.0);

    let c = (a+b).sqrt();

    if c < (c1.get_radius() + c2.get_radius()) {
        println!("hey");
        return true;
    }

    false

}


pub fn rect_vs_circle(r:&Rect,c:&Circle) -> bool {


    let mut testx = c.get_x();
    let mut testy = c.get_y();

    if c.get_x() < r.get_x() {
        testx = r.get_x();
    } else if c.get_x() > r.get_max_x() {
        testx = r.get_max_x();
    }

    if c.get_y() < r.get_y() {
        testy = r.get_y();
    } else if c.get_y() > r.get_max_y() {
        testy = r.get_max_y();
    }

    let distx = c.get_x() - testx;
    let disty = c.get_y() - testy;

    let distance = (distx.powf(2.0)+(disty.powf(2.0))).sqrt();

    if distance <= c.get_radius() {
        return true;
    }

    false

}

pub fn rect_vs_rect_horizontally(r1:&Rect,r2:&Rect,dx:f32) -> bool {
    r1.get_max_x() + dx > r2.get_x() && r1.get_x()  + dx < r2.get_max_x() &&
        r1.get_max_y() > r2.get_y() && r1.get_y() < r2.get_max_y()
}

pub fn rect_vs_rect_vertically(r1:&Rect,r2:&Rect,dy:f32) -> bool {
    r1.get_max_x() > r2.get_x() && r1.get_x() < r2.get_max_x() &&
        r1.get_max_y() + dy > r2.get_y() && r1.get_y() + dy < r2.get_max_y()
}

pub fn rect_vs_rect(r1:&Rect,r2:&Rect) -> bool {

    if r1.get_max_x() > r2.get_x() && r1.get_x() < r2.get_max_x()
        && r1.get_max_y() > r2.get_y() && r1.get_y() < r2.get_max_y() {
        return true;

    }

    false
}


pub fn rect_from_right(r1_before:&Rect,r1_after:&Rect,r2:&Rect) -> bool {
    r1_before.get_x() >= r2.get_max_x() && r1_after.get_x() < r2.get_max_x()
}
pub fn rect_from_left(r1_before:&Rect,r1_after:&Rect,r2:&Rect) -> bool {
    r1_before.get_max_x() < r2.get_x() && r1_after.get_max_x() >= r2.get_x()
}
pub fn rect_from_top(r1_before:&Rect,r1_after:&Rect,r2:&Rect) -> bool {
    r1_before.get_max_y()  < r2.get_y() && r1_after.get_max_y() >= r2.get_y()
}
pub fn rect_from_bottom(r1_before:&Rect,r1_after:&Rect,r2:&Rect) -> bool {
    r1_before.get_max_y() < r2.get_y() && r1_after.get_max_y() > r2.get_y()
}

pub fn rect_vs_point(r1:&Rect,p1:&Vec2) -> bool {
    p1.x >= r1.get_x() && p1.x <= r1.get_max_x() && p1.y >= r1.get_y() && p1.y <= r1.get_max_y()
}









pub fn poly_vs_point(vertex:Vec<Vec2>,pt:&Vec2) -> bool {

    let length = vertex.len();
    let mut next = 0;
    let mut collid = false;

    for i in 0..length {

        next = i + 1;

        // the last vec2 next is the first one
        if next == length  {
            next = 0;
        }

        let vc = vertex[i];
        let vn = vertex[next];


        if ((vc.x > pt.y && vn.y < pt.y) || (vc.y < pt.y && vn.y > pt.y)) &&
            (pt.x < (vn.x - vc.x) * (pt.y-vc.y) / (vn.y - vc.y)+ vc.x) {
            // flip the boolean
            collid = !collid;

        }


    }


    collid

}
