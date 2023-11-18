use crate::{
    common::FP,
    hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

pub trait Material {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}
impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}
impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        *scattered = Ray::new(
            rec.p,
            if scatter_direction.near_zero() {
                rec.normal
            } else {
                scatter_direction
            },
        );
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: FP,
}
impl Metal {
    pub fn new(albedo: Color, fuzz: FP) -> Self {
        Self { albedo, fuzz }
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = r_in.direction.reflect(rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_unit_vector());
        *attenuation = self.albedo;
        scattered.direction.dot(rec.normal) > 0.0
    }
}
