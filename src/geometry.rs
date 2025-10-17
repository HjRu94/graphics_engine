use macroquad::prelude::*;

use ndarray::prelude::*;
use ndarray::Zip;
use std::ops::Add;
#[derive(Clone, PartialEq)]
pub struct Vector3<T>(Array1<T>);

impl<T> Add for Vector3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // Element-wise addition using Zip
        let result = Zip::from(&self.0).and(&rhs.0).map_collect(|a, b| *a + *b);
        Vector3(result)
    }
}

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

#[derive(Clone)]
pub struct Orientation {
    roll: f32,
    pitch: f32,
    yaw: f32,
}

impl Orientation {
    pub const ZERO: Orientation = Orientation {
        roll: 0.0,
        pitch: 0.0,
        yaw: 0.0,
    };

    pub fn new(roll: f32, pitch: f32, yaw: f32) -> Self {
        Orientation {
            roll: roll,
            pitch: pitch,
            yaw: yaw,
        }
    }
    pub fn direction(&self) -> Vector3<f32> {
        self.apply(&Vector3::new(-1.0, 0.0, 0.0))
    }
    pub fn apply(&self, v: &Vector3<f32>) -> Vector3<f32> {
        Self::yaw_matrix(self.yaw)
            .dot(&Self::pitch_matrix(self.pitch))
            .dot(&Self::roll_matrix(self.roll))
            .dot(&v.0)
            .try_into()
            .expect("Dimention is incorrect")
    }
    pub fn unapply(&self, v: &Vector3<f32>) -> Vector3<f32> {
        Self::roll_matrix(-self.roll)
            .dot(&Self::pitch_matrix(-self.pitch))
            .dot(&Self::yaw_matrix(-self.yaw))
            .dot(&v.0)
            .try_into()
            .expect("Dimention is incorrect")
    }
    pub fn roll(&self) -> f32 {
        self.roll
    }
    fn roll_matrix(roll: f32) -> Array2<f32> {
        array![
            [1.0, 0.0, 0.0],
            [0.0, roll.cos(), roll.sin()],
            [0.0, -roll.sin(), roll.cos()],
        ]
    }
    pub fn pitch(&self) -> f32 {
        self.pitch
    }
    fn pitch_matrix(pitch: f32) -> Array2<f32> {
        array![
            [pitch.cos(), 0.0, -pitch.sin()],
            [0.0, 1.0, 0.0],
            [pitch.sin(), 0.0, pitch.cos()],
        ]
    }
    pub fn yaw(&self) -> f32 {
        self.yaw
    }
    fn yaw_matrix(yaw: f32) -> Array2<f32> {
        array![
            [yaw.cos(), yaw.sin(), 0.0],
            [-yaw.sin(), yaw.cos(), 0.0],
            [0.0, 0.0, 1.0],
        ]
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
impl<T> Into<Array1<T>> for Vector3<T> {
    fn into(self) -> Array1<T> {
        self.0
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
    pub fn apply_pose(&self, pose: &Pose) -> Self {
        Triangle {
            p1: pose.orientation().apply(self.p1()) + pose.pos().clone(),
            p2: pose.orientation().apply(self.p2()) + pose.pos().clone(),
            p3: pose.orientation().apply(self.p3()) + pose.pos().clone(),
            normal: pose.orientation().apply(self.normal()),
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

#[derive(Clone)]
pub struct Pose {
    pos: Vector3<f32>,
    orientation: Orientation,
}

impl Pose {
    pub fn zero() -> Self {
        Pose {
            pos: Vector3::zero(),
            orientation: Orientation::ZERO,
        }
    }
    pub fn new(pos: Vector3<f32>, orientation: Orientation) -> Self {
        Pose {
            pos: pos,
            orientation: orientation,
        }
    }
    pub fn pos(&self) -> &Vector3<f32> {
        &self.pos
    }
    pub fn orientation(&self) -> &Orientation {
        &self.orientation
    }
}
