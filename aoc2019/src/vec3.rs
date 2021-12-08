#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vec3 {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
    #[inline(always)]
    pub fn l1_norm(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
    #[inline(always)]
    pub fn get(&self, index: usize) -> i64 {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid index!"),
        }
    }
}
use std::ops::{Add, Sub};
impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn add(self, other: Vec3) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn sub(self, other: Vec3) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn add(self, other: &Vec3) -> Self::Output {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn sub(self, other: &Vec3) -> Self::Output {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
