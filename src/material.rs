use crate::{
    common::FP,
    hittable::HitRecord,
    ray::Ray,
    texture::Texture,
    vec3::{Color, Point3, Vec3},
};

pub trait Material: Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)>;
    fn emitted(&self, _u: FP, _v: FP, _p: &Point3) -> Color {
        Color::ZERO
    }
}

pub struct Lambertian<T: Texture> {
    albedo: T,
}
impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}
impl<T: Texture> Material for Lambertian<T> {
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
        let reflected = ray.direction.reflect(&hit.normal) + self.fuzz * Vec3::random_unit_vector();
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

pub struct DiffuseLight<T: Texture> {
    emit: T,
}
impl<T: Texture> DiffuseLight<T> {
    pub fn new(emit: T) -> Self {
        Self { emit }
    }
}
impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, u: FP, v: FP, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
