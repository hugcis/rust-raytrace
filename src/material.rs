use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{
    color, dot, random_in_unit_sphere, random_unit_vector, reflect, refract, unit_vector, Color,
};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray);
}

pub struct Lambertian {
    color: Color,
}

impl Lambertian {
    pub fn new(col: Color) -> Lambertian {
        Lambertian { color: col }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let mut scatter_direction = rec.get_normal() + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.get_normal();
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.color;
        (true, attenuation, scattered)
    }
}

pub struct Metal {
    color: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(col: Color, f: f64) -> Metal {
        Metal {
            color: col,
            fuzz: match f < 1. {
                true => f,
                false => 1.,
            },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let reflected = reflect(unit_vector(&r_in.direction()), rec.get_normal());
        let scattered = Ray::new(rec.p, reflected + random_in_unit_sphere() * self.fuzz);
        let attenuation = self.color;
        (
            dot(scattered.direction(), rec.get_normal()) > 0.,
            attenuation,
            scattered,
        )
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir: ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let attenuation = color(1., 1., 1.);
        let refraction_ratio = if rec.front_face {
            1. / self.ir
        } else {
            self.ir
        };
        let unit_direction = unit_vector(&r_in.direction());
        let cos_theta = dot(-unit_direction, rec.get_normal()).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let direction = if refraction_ratio * sin_theta > 1.0 {
            reflect(unit_direction, rec.get_normal())
        } else {
            refract(unit_direction, rec.get_normal(), refraction_ratio)
        };
        let scattered = Ray::new(rec.p, direction);
        (true, attenuation, scattered)
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64{
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;

}
