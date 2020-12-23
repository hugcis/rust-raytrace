use crate::vec3::{Point3, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}
