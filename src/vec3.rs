use std::ops;
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub e: [f64; 3]
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
        Vec3 { e: [self.e[0] + other.e[0],
                   self.e[1] + other.e[1],
                   self.e[2] + other.e[2]] }
    }
}
impl ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f64) -> Vec3 {
        Vec3 { e: [self.e[0] + other,
                   self.e[1] + other,
                   self.e[2] + other] }
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        [self.e[0] * other.e[0],
         self.e[1] * other.e[1],
         self.e[2] * other.e[2]].iter().sum()
    }
}
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 { e: [self.e[0] * other,
                   self.e[1] * other,
                   self.e[2] * other] }
    }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Vec3 {
        self + (other * -1_f64)
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
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        self * (1_f64/rhs)
    }
}
impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1_f64/rhs;
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

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn length_squared(self) -> f64 {
        self.e.iter().map(|x| x * x).sum()
    }
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn x(self) -> f64 {
        self.e[0]
    }
    pub fn y(self) -> f64 {
        self.e[1]
    }
    pub fn z(self) -> f64 {
        self.e[2]
    }
}

pub fn dot(one: Vec3, other: Vec3) -> f64 {
    one * other
}

pub fn cross(one: Vec3, other: Vec3) -> Vec3 {
    Vec3 { e: [one.e[1] * other.e[2] - one.e[2] * other.e[1],
               one.e[2] * other.e[0] - one.e[0] * other.e[2],
               one.e[0] * other.e[1] - one.e[1] * other.e[0]]}
}
pub fn unit_vector(v: &Vec3) -> Vec3 {
    v.clone() / v.length()
}

pub fn write_color(v: Vec3, mut writer: impl std::io::Write) -> std::io::Result<()> {
    writer.write(&format!("{} {} {}\n", (255.999 * v.e[0]).floor(),
                         (255.999 * v.e[1]).floor(),
                          (255.999 * v.e[2]).floor()).into_bytes())?;
    Ok(())
}

pub fn color(x: f64, y: f64, z: f64) -> Color {
    Color::new(x, y, z)
}
