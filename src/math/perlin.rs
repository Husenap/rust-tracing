use rand::Rng;

use crate::{common::FP, math::vec3::Vec3};

const POINT_COUNT: i32 = 256;

#[derive(Clone)]
pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        Self {
            ranvec: (0..POINT_COUNT)
                .map(|_| Vec3::random_range(-1.0, 1.0))
                .collect(),
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Vec3) -> FP {
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let u = p.x - i as FP;
        let v = p.y - j as FP;
        let w = p.z - k as FP;

        let mut c = [[[Vec3::ZERO; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }

        Self::trilinear_interpolation(c, u, v, w)
    }

    pub fn turbulence(&self, p: &Vec3, depth: i32) -> FP {
        let mut acc = 0.0;
        let mut p = *p;
        let mut w = 1.0;

        for _ in 0..depth {
            acc += w * self.noise(&p);
            w *= 0.5;
            p *= 2.0;
        }

        acc.abs()
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = (0..POINT_COUNT).collect();

        Self::permute(&mut p, POINT_COUNT);

        p
    }

    fn permute(p: &mut Vec<i32>, n: i32) {
        for i in (1..n).rev() {
            let target = rand::thread_rng().gen_range(0..=i);
            p.swap(i as usize, target as usize);
        }
    }

    fn trilinear_interpolation(c: [[[Vec3; 2]; 2]; 2], u: FP, v: FP, w: FP) -> FP {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut acc = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as FP, v - j as FP, w - k as FP);
                    acc += (i as FP * uu + (1 - i) as FP * (1 as FP - uu))
                        * (j as FP * vv + (1 - j) as FP * (1 as FP - vv))
                        * (k as FP * ww + (1 - k) as FP * (1 as FP - ww))
                        * c[i][j][k].dot(&weight_v);
                }
            }
        }
        acc
    }
}
