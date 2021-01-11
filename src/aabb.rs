use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Copy, Clone, Debug)]
pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> AABB {
        AABB { min, max }
    }

    #[inline]
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        !(0..3).any(|a| {
            let invd = 1_f64 / r.direction()[a];
            let mut t0 = invd * (self.min[a] - r.origin()[a]);
            let mut t1 = invd * (self.max[a] - r.origin()[a]);
            if invd < 0. {
                std::mem::swap(&mut t0, &mut t1);
            };
            let tmin = if t0 > t_min { t0 } else { t_min };
            let tmax = if t1 < t_max { t1 } else { t_max };
            tmax <= tmin
        })
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Point3::new(
        box0.min.x().min(box1.min.x()),
        box0.min.y().min(box1.min.y()),
        box0.min.z().min(box1.min.z()),
    );
    let big = Point3::new(
        box0.max.x().max(box1.max.x()),
        box0.max.y().max(box1.max.y()),
        box0.max.z().max(box1.max.z()),
    );
    AABB::new(small, big)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3::Vec3;

    #[test]
    fn ray_intersects_bbox() {
        let r = Ray::new(Vec3::new(-2., -2., -2.), Vec3::new(1., 1., 1.));
        let bbox = AABB::new(Point3::new(0., 0., 0.), Point3::new(1., 1., 1.));
        assert_eq!(bbox.hit(&r, 0.001, 100.), true);
    }
}
