use std::{cmp::Ordering, sync::Arc};

use rand::Rng;

use crate::{
    hittable::{HitRecord, Hittable, HittableList},
    math::{aabb::AABB, interval::Interval, ray::Ray},
};

pub struct BVHNode {
    root: (Node, AABB),
}

enum Node {
    Branch(Box<(Node, AABB)>, Box<(Node, AABB)>),
    Leaf(Arc<dyn Hittable>),
}

impl BVHNode {
    pub fn new(list: &mut HittableList) -> Self {
        Self::new_from_objects(&mut list.objects)
    }
    pub fn new_from_objects(objects: &mut [Arc<dyn Hittable>]) -> Self {
        Self {
            root: Self::node_from_list(objects),
        }
    }

    fn node_from_list(objects: &mut [Arc<dyn Hittable>]) -> (Node, AABB) {
        let axis: usize = rand::thread_rng().gen_range(0..=2);

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let object_span = objects.len();

        if object_span == 1 {
            let obj = &objects[0];
            (Node::Leaf(Arc::clone(&obj)), obj.bounding_box())
        } else if object_span == 2 {
            let mut left = &objects[0];
            let mut right = &objects[1];
            if comparator(left, right) != Ordering::Less {
                (left, right) = (right, left);
            }
            (
                Node::Branch(
                    Box::new((Node::Leaf(Arc::clone(left)), left.bounding_box())),
                    Box::new((Node::Leaf(Arc::clone(right)), right.bounding_box())),
                ),
                AABB::new_from_aabbs(left.bounding_box(), right.bounding_box()),
            )
        } else {
            objects.sort_unstable_by(comparator);
            let (left_objects, right_objects) = objects.split_at_mut(object_span / 2);
            let left = Self::node_from_list(left_objects);
            let right = Self::node_from_list(right_objects);
            let bbox = AABB::new_from_aabbs(left.1, right.1);
            (Node::Branch(Box::new(left), Box::new(right)), bbox)
        }
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
        if a.bounding_box().axis(axis).min < b.bounding_box().axis(axis).min {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }
    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }
    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

trait NodeHit {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}

impl NodeHit for (Node, AABB) {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if !self.1.hit(r, &ray_t) {
            return None;
        }

        match &self.0 {
            Node::Branch(left, right) => {
                if let Some(hit_left) = left.hit(r, ray_t) {
                    if let Some(hit_right) = right.hit(r, &Interval::new(ray_t.min, hit_left.t)) {
                        Some(hit_right)
                    } else {
                        Some(hit_left)
                    }
                } else if let Some(hit_right) = right.hit(r, ray_t) {
                    Some(hit_right)
                } else {
                    None
                }
            }
            Node::Leaf(obj) => obj.hit(r, ray_t),
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        self.root.hit(r, &ray_t)
    }

    fn bounding_box(&self) -> AABB {
        self.root.1
    }
}
