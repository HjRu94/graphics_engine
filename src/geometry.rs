use macroquad::prelude::*;

use ndarray::prelude::*;
use ndarray::Zip;
use std::ops::{Add, Mul};

use std::hash::{Hash, Hasher};

use crate::view::Camera;

#[derive(Clone)]
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

// Implement scalar multiplication: Vector3<T> * T
impl<T> Mul<T> for Vector3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        let result = self.0.mapv(|x| x * scalar);
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
    pub fn set_x(&mut self, value: T) {
        self.0[0] = value;
    }
    pub fn y(&self) -> T {
        self.0[1]
    }
    pub fn set_y(&mut self, value: T) {
        self.0[1] = value;
    }
    pub fn z(&self) -> T {
        self.0[2]
    }
    pub fn set_z(&mut self, value: T) {
        self.0[2] = value;
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

impl Vector3<f32> {
    pub fn apply_pose(&mut self, pose: &Pose) {
        *self = pose.orientation().apply(self) + pose.pos().clone();
    }
    pub fn camera_project(&mut self, camera: &Camera) {
        let p = &self.0;
        let pos = camera.pos().get_array();
        let orient = camera.orientation();
        let v = p - pos;
        let rot = orient.unapply(&v.try_into().expect("Matrix is of wrong dimention"));

        let scalar = 1.0 / rot.x();

        self.0[0] = rot.x();
        self.0[1] = rot.y() * scalar;
        self.0[2] = rot.z() * scalar;
    }
}
impl PartialEq for Vector3<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len()
            && self
                .0
                .iter()
                .zip(other.0.iter())
                .all(|(a, b)| (a - b).abs() < f32::EPSILON)
    }
}

impl Eq for Vector3<f32> {}

impl Hash for Vector3<f32> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Use bit representation for stable hashing of f32
        for x in self.0.iter() {
            state.write_u32(x.to_bits());
        }
    }
}

#[derive(Clone)]
pub struct Orientation {
    roll: f32,
    pitch: f32,
    yaw: f32,
    rotation_matrix: Array2<f32>, // precomputed R = Yaw * Pitch * Roll
    inverse_matrix: Array2<f32>,  // precomputed R⁻¹ = Roll(-r) * Pitch(-p) * Yaw(-y)
}

impl Orientation {
    pub fn zero() -> Orientation {
        Orientation {
            roll: 0.0,
            pitch: 0.0,
            yaw: 0.0,
            rotation_matrix: array![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            inverse_matrix: array![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    pub fn new(roll: f32, pitch: f32, yaw: f32) -> Self {
        let r_matrix = Self::yaw_matrix(yaw)
            .dot(&Self::pitch_matrix(pitch))
            .dot(&Self::roll_matrix(roll));

        let inv_matrix = Self::roll_matrix(-roll)
            .dot(&Self::pitch_matrix(-pitch))
            .dot(&Self::yaw_matrix(-yaw));

        Orientation {
            roll,
            pitch,
            yaw,
            rotation_matrix: r_matrix,
            inverse_matrix: inv_matrix,
        }
    }

    pub fn direction(&self) -> Vector3<f32> {
        self.apply(&Vector3::new(-1.0, 0.0, 0.0))
    }

    pub fn apply(&self, v: &Vector3<f32>) -> Vector3<f32> {
        self.rotation_matrix
            .dot(&v.0)
            .try_into()
            .expect("Dimension is incorrect")
    }

    pub fn unapply(&self, v: &Vector3<f32>) -> Vector3<f32> {
        self.inverse_matrix
            .dot(&v.0)
            .try_into()
            .expect("Dimension is incorrect")
    }

    fn roll_matrix(roll: f32) -> Array2<f32> {
        array![
            [1.0, 0.0, 0.0],
            [0.0, roll.cos(), roll.sin()],
            [0.0, -roll.sin(), roll.cos()],
        ]
    }

    fn pitch_matrix(pitch: f32) -> Array2<f32> {
        array![
            [pitch.cos(), 0.0, -pitch.sin()],
            [0.0, 1.0, 0.0],
            [pitch.sin(), 0.0, pitch.cos()],
        ]
    }

    fn yaw_matrix(yaw: f32) -> Array2<f32> {
        array![
            [yaw.cos(), yaw.sin(), 0.0],
            [-yaw.sin(), yaw.cos(), 0.0],
            [0.0, 0.0, 1.0],
        ]
    }
    pub fn roll(&self) -> f32 {
        self.roll
    }
    pub fn pitch(&self) -> f32 {
        self.pitch
    }
    pub fn yaw(&self) -> f32 {
        self.yaw
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
    pub fn camera_project(&mut self, camera: &Camera) {
        self.p1.camera_project(camera);
        self.p2.camera_project(camera);
        self.p3.camera_project(camera);
    }
    pub fn apply_pose(&mut self, pose: &Pose) {
        self.p1.apply_pose(pose);
        self.p2.apply_pose(pose);
        self.p3.apply_pose(pose);
        self.normal = pose.orientation().apply(self.normal());
    }
    pub fn normal(&self) -> &Vector3<f32> {
        &self.normal
    }
    pub fn p1(&self) -> &Vector3<f32> {
        &self.p1
    }
    pub fn set_p1(&mut self, value: Vector3<f32>) {
        self.p1 = value;
    }
    pub fn p2(&self) -> &Vector3<f32> {
        &self.p2
    }
    pub fn set_p2(&mut self, value: Vector3<f32>) {
        self.p2 = value;
    }
    pub fn p3(&self) -> &Vector3<f32> {
        &self.p3
    }
    pub fn set_p3(&mut self, value: Vector3<f32>) {
        self.p3 = value;
    }
}

#[derive(Clone)]
pub struct Pose {
    pos: Vector3<f32>,
    orientation: Orientation,
}

impl std::fmt::Display for Pose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Pose:\n  pos: (x: {}, y: {}, z: {})\n  rotation: (roll: {}, pitch: {}, yaw: {})",
            self.pos.x(),
            self.pos.y(),
            self.pos.z(),
            self.orientation.roll(),
            self.orientation.pitch(),
            self.orientation.yaw(),
        )
    }
}

impl Pose {
    pub fn zero() -> Self {
        Pose {
            pos: Vector3::zero(),
            orientation: Orientation::zero(),
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
