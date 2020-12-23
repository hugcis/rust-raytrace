use crate::ray::Ray;
use crate::vec3::{cross, Point3, Vec3, unit_vector, random_in_unit_sphere};
use std::f64::consts;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = consts::PI * vfov / 180.;
        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w = unit_vector(&(lookfrom - lookat));
        let u = unit_vector(&cross(vup, w));
        let v = cross(w, u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - w * focus_dist;
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius: aperture / 2.
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = random_in_unit_sphere() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset,
        )
    }
}
