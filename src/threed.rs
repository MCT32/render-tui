use std::ops::{Add, Sub, Mul, Div};

use crate::canvas::Canvas;

#[derive(Clone, Copy)]
pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

impl Float2 {
    // TODO: Covert to macro
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl From<(f32, f32)> for Float2 {
    fn from(from: (f32, f32)) -> Self {
        Self { x: from.0, y: from.1 }
    }
}

impl Add<Float2> for Float2 {
    type Output = Self;

    fn add(self, right: Float2) -> <Self as Add<Float2>>::Output {
        Self { x: self.x + right.x, y: self.y + right.y }
    }
}

impl Sub<Float2> for Float2 {
    type Output = Self;

    fn sub(self, right: Float2) -> <Self as Sub<Float2>>::Output {
        Self { x: self.x - right.x, y: self.y - right.y }
    }
}

impl Mul<Float2> for Float2 {
    type Output = Self;

    fn mul(self, right: Float2) -> <Self as Mul<Float2>>::Output {
        Self { x: self.x * right.x, y: self.y * right.y }
    }
}

impl Div<f32> for Float2 {
    type Output = Self;

    fn div(self, right: f32) -> <Self as Div<f32>>::Output {
        Self { x: self.x / right, y: self.y / right }
    }
}

#[derive(Clone, Copy)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Float3 {
    // TODO: Covert to macro
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl From<(f32, f32, f32)> for Float3 {
    fn from(from: (f32, f32, f32)) -> Self {
        Self { x: from.0, y: from.1, z: from.2 }
    }
}

impl Add<Float3> for Float3 {
    type Output = Self;

    fn add(self, right: Float3) -> <Self as Add<Float3>>::Output {
        Self { x: self.x + right.x, y: self.y + right.y, z: self.z + right.z }
    }
}

impl Sub<Float3> for Float3 {
    type Output = Self;

    fn sub(self, right: Float3) -> <Self as Sub<Float3>>::Output {
        Self { x: self.x - right.x, y: self.y - right.y, z: self.z + right.z }
    }
}

impl Mul<Float3> for Float3 {
    type Output = Self;

    fn mul(self, right: Float3) -> <Self as Mul<Float3>>::Output {
        Self { x: self.x * right.x, y: self.y * right.y, z: self.z + right.z }
    }
}

impl Div<f32> for Float3 {
    type Output = Self;

    fn div(self, right: f32) -> <Self as Div<f32>>::Output {
        Self { x: self.x / right, y: self.y / right, z: self.z / right }
    }
}

pub struct Model {
    verts: Vec<Float3>,
    lines: Vec<(usize, usize)>,

    rotation: Float3,
}

impl Model {
    pub fn draw(&self, camera: &Camera, canvas: &mut Canvas) {
        for vert in &self.verts {
            let coord = camera.world_to_screen(vert);
            canvas.plot(coord, 'o');
        }
    }

    pub fn set_rotation(&mut self, rotation: Float3) {self.rotation = rotation;}

    pub fn cube(start: Float3, end: Float3) -> Self {
        Self {
            verts: vec![
                Float3::new(start.x, start.y, start.z),
                Float3::new(end.x,   start.y, start.x),
                Float3::new(end.x,   end.y,   start.x),
                Float3::new(start.x, end.y,   start.x),
                Float3::new(start.x, start.y, end.x  ),
                Float3::new(end.x,   start.y, end.x  ),
                Float3::new(end.x,   end.y,   end.x  ),
                Float3::new(start.x, end.y,   end.x  ),
            ],
            lines: vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 0),
                (4, 5),
                (5, 6),
                (6, 7),
                (7, 4),
                (0, 4),
                (1, 5),
                (2, 6),
                (3, 7),
            ],
            rotation: Float3::zero()
        }
    }
}

pub struct Camera {
    aspect_ratio: f32,
    fov: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32, fov: f32) -> Self {
        Self{ aspect_ratio, fov }
    }

    pub fn world_to_screen(&self, vert: &Float3) -> Float2 {
        Float2::from((vert.x, vert.z)) / self.fov
    }
}

