use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{dot, color, random_unit_vector, reflect, unit_vector, Color};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, &Color, &Ray);
}

pub struct Lambertian {
    color: Color,
}

impl Lambertian {
    pub fn new() -> Lambertian {
        Lambertian {
            color: color(1., 1., 1.),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, &Color, &Ray) {
        let mut scatter_direction = rec.get_normal() + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.get_normal();
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.color;
        (true, &attenuation, &scattered)
    }
}

pub struct Metal {
    color: Color,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, &Color, &Ray) {
        let mut reflected = reflect(unit_vector(&r_in.direction()), rec.get_normal());
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.color;
        (dot(scattered.direction(), rec.get_normal()) > 0.,
         &attenuation, &scattered)
    }
}
