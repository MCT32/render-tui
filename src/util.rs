use std::num::TryFromIntError;
use std::ops::{Add, Div};

#[derive(Copy, Clone)]
pub struct ScreenCoord {
    pub x: i16,
    pub y: i16,
}

impl ScreenCoord {
    pub fn new(x: i16, y: i16) -> Self {
        Self {x,y}
    }
}

impl TryFrom<(u16, u16)> for ScreenCoord {
    type Error = TryFromIntError;

    fn try_from(input: (u16, u16)) -> Result<Self, Self::Error> {
        Ok(Self {x: input.1.try_into()?, y: input.0.try_into()?})
    }
}

impl From<(i16, i16)> for ScreenCoord {
    fn from(input: (i16, i16)) -> Self {
        Self {x: input.1, y: input.0}
    }
}

impl Add<(i16, i16)> for ScreenCoord {
    type Output = Self;

    fn add(self, i: (i16, i16)) -> <Self as Add<(i16, i16)>>::Output {
        Self {
            x: self.x + i.1,
            y: self.y + i.0,
        }
    }
}

impl Div<i16> for ScreenCoord {
    type Output = Self;

    fn div(self, i: i16) -> <Self as Div<i16>>::Output {
        Self {
            x: self.x / i,
            y: self.y / i,
        }
    }
}

