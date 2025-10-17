use macroquad::prelude::*;

use ndarray::Array1;
#[derive(Clone, PartialEq)]
pub struct Vector3<T>(Array1<T>);

impl<T: Copy> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector3(Array1::from(vec![x, y, z]))
    }
    pub fn x(&self) -> T {
        self.0[0]
    }
    pub fn y(&self) -> T {
        self.0[1]
    }
    pub fn z(&self) -> T {
        self.0[2]
    }
    pub fn get_array(&self) -> &Array1<T> {
        &self.0
    }
}

impl<T> Vector3<T>
where
    T: From<u8> + Copy,
{
    pub fn zero() -> Self {
        Vector3::new(0u8.into(), 0u8.into(), 0u8.into())
    }
}

impl<T: std::fmt::Display + Copy> std::fmt::Display for Vector3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}
#[derive(Debug)]
pub enum ConvertVectorError {
    OutOfRangeError,
}
impl<T> TryFrom<Array1<T>> for Vector3<T> {
    type Error = ConvertVectorError;
    fn try_from(value: Array1<T>) -> Result<Self, Self::Error> {
        match value.len() {
            3 => Ok(Vector3::<T>(value)),
            _ => Err(ConvertVectorError::OutOfRangeError),
        }
    }
}

#[derive(Clone)]
pub struct Triangle {
    p1: Vector3<f32>,
    p2: Vector3<f32>,
    p3: Vector3<f32>,
    normal: Vector3<f32>,
}

impl std::fmt::Display for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Triangle: \n p1: {} \n p2: {} \n p3: {} \n normal: {}",
            self.p1, self.p2, self.p3, self.normal
        )
    }
}

impl Triangle {
    pub fn new(p1: Vector3<f32>, p2: Vector3<f32>, p3: Vector3<f32>, normal: Vector3<f32>) -> Self {
        Triangle {
            p1: p1,
            p2: p2,
            p3: p3,
            normal: normal,
        }
    }
    pub fn normal(&self) -> &Vector3<f32> {
        &self.normal
    }
    pub fn p1(&self) -> &Vector3<f32> {
        &self.p1
    }
    pub fn p2(&self) -> &Vector3<f32> {
        &self.p2
    }
    pub fn p3(&self) -> &Vector3<f32> {
        &self.p3
    }
}

pub struct Pose {
    // pos : a vector in euclidian space
    pos: Vector3<f32>,
    // orientation: a vector containing radiant values for roll pitch yaw
    orientation: Vector3<f32>,
}

impl Pose {
    pub fn new(pos: Vector3<f32>, orientation: Vector3<f32>) -> Self {
        Pose {
            pos: pos,
            orientation: orientation,
        }
    }
    pub fn pos(&self) -> &Vector3<f32> {
        &self.pos
    }
    pub fn orientation(&self) -> &Vector3<f32> {
        &self.orientation
    }
}
