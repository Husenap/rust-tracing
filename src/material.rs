use std::sync::Arc;

use crate::{
    common::FP,
    hittable::HitRecord,
    math::{
        ray::Ray,
        vec3::{Color, Point3, Vec3},
    },
    texture::Texture,
};

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)>;
    fn emitted(&self, _u: FP, _v: FP, _p: &Point3) -> Color {
        Color::ZERO
    }
}

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}
impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}
impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = hit.normal + Vec3::random_unit_vector();
        Some((
            Ray::new(
                hit.p,
                if scatter_direction.near_zero() {
                    hit.normal
                } else {
                    scatter_direction
                },
            )
            .with_time(ray.time),
            self.albedo.value(hit.u, hit.v, &hit.p),
        ))
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
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray.direction.normalize().reflect(&hit.normal)
            + self.fuzz * Vec3::random_in_unit_sphere();
        if reflected.dot(&hit.normal) > 0.0 {
            let scattered = Ray::new(hit.p, reflected).with_time(ray.time);
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: FP,
}
impl Dielectric {
    pub fn new(ir: FP) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: FP, ref_idx: FP) -> FP {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}
impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::ONE;
        let refraction_ratio = if hit.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray.direction.normalize();
        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if refraction_ratio * sin_theta > 1.0
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rand::random()
        {
            unit_direction.reflect(&hit.normal)
        } else {
            unit_direction.refract(&hit.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit.p, direction).with_time(ray.time);
        Some((scattered, attenuation))
    }
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}
impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> Self {
        Self { emit }
    }
}
impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, u: FP, v: FP, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}
impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}
impl Material for Isotropic {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let scattered = Ray::new(hit.p, Vec3::random_unit_vector()).with_time(ray.time);
        let attenuation = self.albedo.value(hit.u, hit.v, &hit.p);
        Some((scattered, attenuation))
    }
}
