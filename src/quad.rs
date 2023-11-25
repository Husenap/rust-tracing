use std::sync::Arc;

use crate::{
    aabb::AABB,
    common::FP,
    hittable::{HitRecord, Hittable, HittableList},
    material::Material,
    vec3::{Point3, Vec3},
};

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: AABB,
    d: FP,
    normal: Vec3,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let n = u.cross(&v);
        let normal = n.normalize();
        let d = normal.dot(&q);
        let w = n / n.length_squared();

        Self {
            q,
            u,
            v,
            w,
            mat,
            bbox: Self::create_bounding_box(&q, &u, &v),
            d,
            normal,
        }
    }

    fn create_bounding_box(q: &Vec3, u: &Vec3, v: &Vec3) -> AABB {
        AABB::new_from_points(*q, *q + *u + *v).pad()
    }

    pub fn cube(a: &Point3, b: &Point3, mat: Arc<dyn Material>) -> Arc<HittableList> {
        let mut sides = HittableList::default();

        let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

        let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y - min.y, 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z - min.z);

        sides.add(Arc::new(Quad::new(
            Point3::new(min.x, min.y, max.z),
            dx,
            dy,
            Arc::clone(&mat),
        )));
        sides.add(Arc::new(Quad::new(
            Point3::new(max.x, min.y, max.z),
            -dz,
            dy,
            Arc::clone(&mat),
        )));
        sides.add(Arc::new(Quad::new(
            Point3::new(max.x, min.y, min.z),
            -dx,
            dy,
            Arc::clone(&mat),
        )));
        sides.add(Arc::new(Quad::new(
            Point3::new(min.x, min.y, min.z),
            dz,
            dy,
            Arc::clone(&mat),
        )));
        sides.add(Arc::new(Quad::new(
            Point3::new(min.x, max.y, max.z),
            dx,
            -dz,
            Arc::clone(&mat),
        )));
        sides.add(Arc::new(Quad::new(
            Point3::new(min.x, min.y, min.z),
            dx,
            dz,
            mat,
        )));

        Arc::new(sides)
    }
}

impl Hittable for Quad {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: &crate::interval::Interval,
    ) -> Option<crate::hittable::HitRecord> {
        let denom = self.normal.dot(&r.direction);

        /*
         * back-face culling for quads?
        if denom > 1e-8 {
            return None;
        }
        */
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.dot(&r.origin)) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);

        let planar_hit_point = intersection - self.q;
        let alpha = self.w.dot(&planar_hit_point.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hit_point));

        if alpha < 0.0 || alpha > 1.0 || beta < 0.0 || beta > 1.0 {
            return None;
        }

        Some(
            HitRecord::new(intersection, self.mat.as_ref(), t, r, self.normal)
                .with_uvs(alpha, beta),
        )
    }

    fn bounding_box(&self) -> crate::aabb::AABB {
        self.bbox
    }
}
