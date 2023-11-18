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

pub struct Dielectric {
    ir: FP,
}
impl Dielectric {
    pub fn new(ir: FP) -> Self {
        Self { ir }
    }
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::ONE;
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction.normalize();
        let refracted = unit_direction.refract(rec.normal, refraction_ratio);

        *scattered = Ray::new(rec.p, refracted);
        true
    }
}
