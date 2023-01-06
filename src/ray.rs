use crate::prelude::*;

pub struct Ray {
    pub orig: Vector3<f64>,
    pub dir: Vector3<f64>,
}

impl Ray {
    pub fn new(orig: Vector3<f64>, dir: Vector3<f64>) -> Self {
        Self {
            orig: orig,
            dir: dir,
        }
    }

    pub fn at(&self, t: f64) -> Vector3<f64> {
        return self.orig + t * self.dir;
    }
}
