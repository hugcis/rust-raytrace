use crate::aabb::{surrounding_box, AABB};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use rand::Rng;
use std::cmp::Ordering;

pub struct BVHNode {
    left: Box<dyn Hittable + Send + Sync>,
    right: Option<Box<dyn Hittable + Send + Sync>>,
    aabb_box: AABB,
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.aabb_box.hit(r, t_min, t_max) {
            None
        } else {
            let hit_left = self.left.hit(r, t_min, t_max);
            let ref_right = self.right.as_ref();
            match hit_left {
                Some(v) => match &self.right {
                    Some(right) => Some(right.hit(r, t_min, v.t).unwrap_or(v)),
                    None => Some(v),
                },
                None => ref_right.and_then(|a| a.hit(r, t_min, t_max)),
            }
        }
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.aabb_box)
    }
}

pub struct HittableItem {
    pub it: Box<dyn Hittable + Send + Sync>,
}

impl BVHNode {
    pub fn new(mut objs: Vec<HittableItem>, time0: f64, time1: f64) -> BVHNode {
        let mut rng = rand::thread_rng();
        let axis = rng.gen_range(0..2);

        let obj_span = objs.len();
        let (left, right) = if obj_span == 1 {
            let last = objs.pop().unwrap();
            (last, None)
        } else if obj_span == 2 {
            let box_a = objs[0].it.bounding_box(0., 0.);
            let box_b = objs[1].it.bounding_box(0., 0.);
            let test = match (box_a, box_b) {
                (Some(v1), Some(v2)) => v1.min.e[axis].partial_cmp(&v2.min.e[axis]).unwrap(),
                (_, _) => panic!("No bounding box found"),
            };
            let last = objs.pop().unwrap();
            let bef = objs.pop().unwrap();
            match test {
                Ordering::Less => (bef, Some(last)),
                _ => (last, Some(bef)),
            }
        } else {
            objs.sort_by(|a, b| {
                let box_a = a.it.bounding_box(0., 0.);
                let box_b = b.it.bounding_box(0., 0.);
                match (box_a, box_b) {
                    (Some(v1), Some(v2)) => v1.min.e[axis].partial_cmp(&v2.min.e[axis]).unwrap(),
                    (_, _) => panic!("No bounding box found"),
                }
            });
            let mid = obj_span / 2;
            let second_half = objs.split_off(mid);
            let lef: Box<dyn Hittable + Send + Sync> = Box::new(BVHNode::new(objs, time0, time1));
            let rig: Box<dyn Hittable + Send + Sync> =
                Box::new(BVHNode::new(second_half, time0, time1));
            (HittableItem { it: lef }, Some(HittableItem { it: rig }))
        };
        let box_a = left.it.bounding_box(time0, time1);
        let box_b = match &right {
            Some(r) => r.it.bounding_box(time0, time1),
            None => None,
        };

        match (box_a, box_b) {
            (Some(v1), Some(v2)) => BVHNode {
                left: left.it,
                right: right.map(|r| r.it),
                aabb_box: surrounding_box(&v1, &v2),
            },
            (Some(v1), None) => BVHNode {
                left: left.it,
                right: None,
                aabb_box: v1,
            },
            (_, _) => panic!("No bounding box"),
        }
    }
}
