use crate::aabb::{surrounding_box, AABB};
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::ray::Ray;
use rand::Rng;
use std::cmp::Ordering;

struct CompareHittable {
    val: Box<dyn Hittable>,
    axis: usize,
}

impl CompareHittable {
    pub fn new(val: Box<dyn Hittable>, axis: usize) -> CompareHittable {
        CompareHittable { val, axis }
    }
}
impl PartialEq for CompareHittable {
    fn eq(&self, other: &Self) -> bool {
        let box_a = self.val.bounding_box(0., 0.);
        let box_b = other.val.bounding_box(0., 0.);
        match (box_a, box_b) {
            (Some(v1), Some(v2)) => v1.min.e[self.axis] == v2.min.e[self.axis],
        }
    }
}
impl Eq for CompareHittable {}
impl PartialOrd for CompareHittable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let box_a = self.val.bounding_box(0., 0.);
        let box_b = other.val.bounding_box(0., 0.);
        match (box_a, box_b) {
            (Some(v1), Some(v2)) => v1.min.e[self.axis].partial_cmp(&v2.min.e[self.axis]),
        }
    }
}

struct BVHNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    aabb_box: AABB,
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.aabb_box.hit(r, t_min, t_max) {
            false
        } else {
            let hit_left = self.left.hit(r, t_min, t_max, rec);
            let hit_right = self
                .left
                .hit(r, t_min, if hit_left { rec.t } else { t_max }, rec);
            hit_left | hit_right
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        Some(self.aabb_box)
    }
}

impl BVHNode {
    pub fn new(
        objs: &mut [Box<dyn Hittable>],
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> BVHNode {
        let mut rng = rand::thread_rng();
        let axis = rng.gen_range(0..2);

        let obj_span = end - start;
        let (left, right) = if obj_span == 1 {
            (&objs[start], &objs[start])
        } else if obj_span == 2 {
            let box_a = objs[start].bounding_box(0., 0.);
            let box_b = objs[start + 1].bounding_box(0., 0.);
            let test = match (box_a, box_b) {
                (Some(v1), Some(v2)) => v1.min.e[axis].partial_cmp(&v2.min.e[axis]).unwrap(),
                (_, _) => panic!("No bounding box found"),
            };
            match test {
                Ordering::Less => (&objs[start], &objs[start + 1]),
                _ => (&objs[start + 1], &objs[start]),
            }
        } else {
            objs[start..end].sort_by(|a, b| {
                let box_a = a.bounding_box(0., 0.);
                let box_b = b.bounding_box(0., 0.);
                match (box_a, box_b) {
                    (Some(v1), Some(v2)) => v1.min.e[axis].partial_cmp(&v2.min.e[axis]).unwrap(),
                    (_, _) => panic!("No bounding box found"),
                }
            });
            let mid = start + obj_span / 2;
            let lef: &Box<dyn Hittable> = &Box::new(BVHNode::new(objs, start, mid, time0, time1));
            let rig: &Box<dyn Hittable> = &Box::new(BVHNode::new(objs, mid, end, time0, time1));
            (lef, rig)
        };
        let box_a = left.bounding_box(0., 0.);
        let box_b = right.bounding_box(0., 0.);

        match (box_a, box_b) {
            (Some(v1), Some(v2)) => BVHNode {
                left: *left,
                right: *right,
                aabb_box: surrounding_box(&v1, &v2),
            },
            (_, _) => panic!("No bounding box"),
        }
    }
}

