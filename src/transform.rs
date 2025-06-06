use crate::vector::Float3;

pub struct Transform {
    //pub scale: Scale,
    pub rotation: Rotation,
    //pub translation: Translation,
}

impl Transform {
    pub fn apply(&self, point: Float3) -> Float3 {
        //let point = self.scale.apply(point);
        let point = self.rotation.apply(point);
        //let point = self.translation.apply(point);

        point
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            rotation: Default::default(),
        }
    }
}

pub struct Rotation {
    pub euler_angles: Float3,
}

impl Rotation {
    pub fn apply(&self, point: Float3) -> Float3 {
        // YXZ order, might change

        // Y
        let point = Float3::new(
            point.x * self.euler_angles.y.cos() + point.z * -self.euler_angles.y.sin(),
            point.y,
            point.x * self.euler_angles.y.sin() + point.z * self.euler_angles.y.cos(),
        );

        // X
        let point = Float3::new(
            point.x,
            point.y * self.euler_angles.x.cos() + point.z * self.euler_angles.x.sin(),
            point.y * -self.euler_angles.x.sin() + point.z * self.euler_angles.x.cos(),
        );

        // Z
        let point = Float3::new(
            point.x * self.euler_angles.z.cos() + point.y * self.euler_angles.z.sin(),
            point.x * -self.euler_angles.z.sin() + point.y * self.euler_angles.z.cos(),
            point.z,
        );

        point
    }
}

impl Default for Rotation {
    fn default() -> Self {
        Self {
            euler_angles: Float3::zero(),
        }
    }
}

impl From<Float3> for Rotation {
    fn from(from: Float3) -> Self { Self { euler_angles: from } }
}

