use crate::aabb::{surrounding_box, AABB};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Option<Box<dyn Material + Send + Sync>>,
}

impl HitRecord {
    pub fn new(
        r: &Ray,
        p: Vec3,
        normal: Vec3,
        t: f64,
        mat: impl Material + Sync + Send + 'static,
    ) -> HitRecord {
        let mut new_ht = HitRecord {
            p,
            t,
            normal: Vec3::new(0., 0., 0.),
            front_face: true,
            mat: Some(Box::new(mat)),
        };
        new_ht.set_face_normal(r, normal);
        new_ht
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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}

pub struct Sphere<T>
where
    T: Material + Copy,
{
    center: Point3,
    radius: f64,
    material: T,
}

impl<T> Sphere<T>
where
    T: Material + Copy,
{
    pub fn new(center: Vec3, radius: f64, material: T) -> Sphere<T> {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<T> Hittable for Sphere<T>
where
    T: 'static + Material + Send + Sync + Copy,
{
    #[inline]
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0. {
            None
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
                let p = r.at(root);
                let outward_normal = (p - self.center) / self.radius;
                let rec = HitRecord::new(r, p, outward_normal, root, self.material);
                Some(rec)
            } else {
                None
            }
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn new(objs: Vec<Box<dyn Hittable + Send + Sync>>) -> HittableList {
        HittableList { objects: objs }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let temp_rec: Option<HitRecord> = None;
        for obj in self.objects.iter() {
            if let Some(temp_rec) = (*obj).hit(r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
            }
        }
        temp_rec
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            None
        } else {
            let mut first_box = true;
            // Useless init since the first loop should go through setting res_box = val
            let mut res_box = AABB::new(Point3::new(0., 0., 0.), Point3::new(0., 0., 0.));
            if self.objects.iter().any(|obj| {
                obj.bounding_box(time0, time1).map_or(false, |val| {
                    res_box = if first_box {
                        val
                    } else {
                        surrounding_box(&res_box, &val)
                    };
                    first_box = false;
                    true
                })
            }) {
                Some(res_box)
            } else {
                None
            }
        }
    }
}
