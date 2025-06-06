use std::ops::{Add, Sub, Mul, Div};

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

impl Div<Float2> for Float2 {
    type Output = Self;

    fn div(self, right: Float2) -> <Self as Div<Float2>>::Output {
        Self { x: self.x / right.x, y: self.y / right.y }
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

impl Div<Float3> for Float3 {
    type Output = Self;

    fn div(self, right: Float3) -> <Self as Div<Float3>>::Output {
        Self { x: self.x / right.x, y: self.y / right.y, z: self.z / right.z }
    }
}

impl Div<f32> for Float3 {
    type Output = Self;

    fn div(self, right: f32) -> <Self as Div<f32>>::Output {
        Self { x: self.x / right, y: self.y / right, z: self.z / right }
    }
}

