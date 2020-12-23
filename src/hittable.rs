use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};
use crate::material::Material;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    normal: Vec3,
    t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Vec3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            t: 0.0,
            front_face: true,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.;
        self.normal = match self.front_face {
            true => outward_normal,
            false => -outward_normal,
        };
    }
    pub fn get_normal(&self) -> Vec3 {
        self.normal
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_rec: &mut HitRecord) -> bool;
    fn get_material(&self) -> &Box<dyn Material>;
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn get_material(&self) -> &Box<dyn Material> {
        &self.material
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            false
        } else {
            let sq_dis = discriminant.sqrt();
            let mut root = (-half_b - sq_dis) / a;
            let has_hit = if (root < t_min) | (root > t_max) {
                root = (-half_b + sq_dis) / a;
                !((root < t_min) | (root > t_max))
            } else {
                true
            };
            if has_hit {
                rec.t = root;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
            }
            has_hit
        }
    }
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objs: Vec<Box<dyn Hittable>>) -> HittableList {
        HittableList { objects: objs }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> (bool, usize) {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut ret_idx = 0;

        for (idx, obj) in self.objects.iter().enumerate() {
            if (*obj).hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                ret_idx = idx;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        (hit_anything, ret_idx)
    }
}
