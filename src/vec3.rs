use crate::utils::{random_double, random_double_range};
use std::fmt;
use std::ops;

const GAMMA: f64 = 2.;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub e: [f64; 3],
}
pub type Color = Vec3;
pub type Point3 = Vec3;

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}
impl ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] + other, self.e[1] + other, self.e[2] + other],
        }
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
        }
    }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}
impl ops::Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: f64) -> Vec3 {
        self + (-1_f64 * other)
    }
}
impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        for i in &mut self.e {
            *i *= rhs;
        }
    }
}
impl ops::AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        for i in &mut self.e {
            *i += rhs;
        }
    }
}
impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        for i in 0..3 {
            self.e[i] += other.e[i];
        }
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        self * (1_f64 / rhs)
    }
}
impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1_f64 / rhs;
    }
}
impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}
impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1_f64
    }
}
impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        ((self.e[0] - other.e[0]).abs() == 0.)
            & ((self.e[1] - other.e[1]).abs() == 0.)
            & ((self.e[2] - other.e[2]).abs() == 0.)
    }
}
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }
    pub fn random() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }
    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    // pub fn z(&self) -> f64 {
    // self.e[2]
    // }
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.e[0].abs() < s) & (self.e[1].abs() < s) & (self.e[2].abs() < s)
    }
}

pub fn dot(one: Vec3, other: Vec3) -> f64 {
    one.e[0] * other.e[0] + one.e[1] * other.e[1] + one.e[2] * other.e[2]
}

pub fn cross(one: Vec3, other: Vec3) -> Vec3 {
    Vec3 {
        e: [
            one.e[1] * other.e[2] - one.e[2] * other.e[1],
            one.e[2] * other.e[0] - one.e[0] * other.e[2],
            one.e[0] * other.e[1] - one.e[1] * other.e[0],
        ],
    }
}

#[inline]
pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

#[inline]
pub fn write_color(
    v: Vec3,
    sample_per_pixel: i32,
    mut writer: impl std::io::Write,
) -> std::io::Result<()> {
    let scale = 1. / f64::from(sample_per_pixel);
    writer.write_all(
        &format!(
            "{} {} {}\n",
            (255.999 * (scale * v.e[0]).powf(1. / GAMMA)).floor(),
            (255.999 * (scale * v.e[1]).powf(1. / GAMMA)).floor(),
            (255.999 * (scale * v.e[2]).powf(1. / GAMMA)).floor()
        )
        .into_bytes(),
    )?;
    Ok(())
}

pub fn color(x: f64, y: f64, z: f64) -> Color {
    Color::new(x, y, z)
}

pub fn point3(x: f64, y: f64, z: f64) -> Point3 {
    Point3::new(x, y, z)
}

#[inline]
pub fn random_in_unit_sphere() -> Vec3 {
    let mut p;
    loop {
        p = Vec3::random_range(-1., 1.);
        if p.length_squared() < 1. {
            break;
        }
    }
    p
}

#[inline]
pub fn random_unit_vector() -> Vec3 {
    unit_vector(&random_in_unit_sphere())
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * dot(v, n) * 2.
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(-uv, n).min(1.0);
    let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
    let r_out_parallel = n * (-(1.0 - r_out_perp.length_squared()).abs().sqrt());
    r_out_perp + r_out_parallel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_vec_ops() {
        assert_eq!(
            Vec3::new(1., 2., 3.) + Vec3::new(4., 5., 6.),
            Vec3::new(5., 7., 9.)
        );
        assert_eq!(
            Vec3::new(1., 2., 3.) * Vec3::new(4., 5., 6.),
            Vec3::new(4., 10., 18.)
        );
        assert_eq!(
            Vec3::new(1., 2., 3.) - Vec3::new(4., 5., 6.),
            Vec3::new(-3., -3., -3.)
        );
    }

    #[test]
    fn vec_f64_ops() {
        assert_eq!(
            Vec3::new(1., 2., 3.) + 4.,
            Vec3::new(5., 6., 7.)
        );
        assert_eq!(
            Vec3::new(1., 2., 3.) * 4.,
            Vec3::new(4., 8., 12.)
        );
    }
}
