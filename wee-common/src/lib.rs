use serde::{Deserialize, Serialize};
use std::error::Error;

pub const PROJECTION_WIDTH: f32 = 1600.0;
pub const PROJECTION_HEIGHT: f32 = 900.0;

pub type WeeResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

pub trait Warn {
    fn warn(self) -> Self;
}

impl<T> Warn for WeeResult<T> {
    fn warn(self) -> Self {
        if let Err(error) = &self {
            log::warn!("{}", error);
        }
        self
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
    pub fn zero() -> Vec2 {
        Vec2::new(0.0, 0.0)
    }
}
impl std::ops::Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}
impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl std::ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl std::ops::SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Mul<Vec2> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl std::ops::Mul<Vec2> for f32 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self * rhs.x, self * rhs.y)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f32) -> Vec2 {
        Vec2::new(self.x / rhs, self.y / rhs)
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    pub fn width(self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(self) -> f32 {
        self.max.y - self.min.y
    }

    pub fn move_position(self, pos: Vec2) -> AABB {
        AABB {
            min: self.min + pos,
            max: self.max + pos,
        }
    }

    pub fn to_rect(self) -> Rect {
        Rect {
            x: self.min.x + self.width() / 2.0,
            y: self.min.y + self.height() / 2.0,
            w: self.width(),
            h: self.height(),
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Rect {
    pub x: f32, // centre x
    pub y: f32, // centre y
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect { x, y, w, h }
    }

    pub fn top_left(self) -> Vec2 {
        let left = self.x - self.w / 2.0;
        let top = self.y - self.h / 2.0;
        Vec2::new(left, top)
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Colour {
    pub fn rgb(r: f32, g: f32, b: f32) -> Colour {
        Colour { r, g, b, a: 1.0 }
    }

    pub fn white() -> Colour {
        Colour::rgb(1.0, 1.0, 1.0)
    }

    pub fn dull_grey() -> Colour {
        Colour::rgb(0.2, 0.2, 0.2)
    }

    pub fn black() -> Colour {
        Colour::rgb(0.0, 0.0, 0.0)
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Size {
        Size { width, height }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Flip {
    pub horizontal: bool,
    pub vertical: bool,
}

impl Default for Flip {
    fn default() -> Flip {
        Flip {
            horizontal: false,
            vertical: false,
        }
    }
}
