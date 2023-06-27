use std::ops::{Add, Sub, Div, Mul,Neg,AddAssign,MulAssign, SubAssign};


#[derive(Debug, Copy, Clone)]
pub struct Vec2 {

    pub x: f32,
    pub y: f32

}

impl Vec2 {

    pub fn new(x:f32,y:f32) -> Self { Self {x,y} }

}

impl Add for Vec2 {

    type Output = Self;

    fn add(self, other: Self) -> Self::Output {

        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }


    }


}

impl AddAssign for Vec2 {

    fn add_assign(&mut self, other: Self) {

        self.x += other.x;
        self.y += other.y;

    }

}

impl Sub for Vec2 {

    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }


}


impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {

        self.x -= other.x;
        self.y -= other.y;

    }

}


impl Div for Vec2 {

    type Output = Self;

    fn div(self, other: Self) -> Self::Output {

        Self {
            x: self.x / other.x,
            y: self.y / other.y
        }

    }

}

impl Div<f32> for Vec2 {

    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        Self {

            x: self.x / other,
            y: self.y / other

        }
    }

}




impl Mul for Vec2 {

    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {

        Self {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }

}

impl Mul<f32> for Vec2 {

    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other
        }
    }

}

impl Mul<f64> for Vec2 {

    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {

        Self {

            x: self.x * other as f32,
            y: self.y * other as f32

        }

    }



}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, other: f32) {
        self.x = self.x * other;
        self.y = self.y * other;
    }
    
}

impl Neg for Vec2 {

    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -(self.x),
            y: -(self.y)
        }
    }

}