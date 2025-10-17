use crate::draw::Drawable;
use macroquad::prelude::*;
use std::f64::consts;
use std::fmt::Formatter;
use std::ops::{Add, Mul};

use ndarray::Array1;

#[derive(Clone, Copy, PartialEq)]
pub struct EucPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Drawable for EucPoint {
    fn draw(&self) {
        let new_point = SphPoint::from(*self);
        draw_circle(
            (new_point.az.expect("HEJ") * 2.0 / consts::PI * 500.0 + 500.0) as f32,
            (new_point.po.expect("HEJ") * 2.0 / consts::PI * 500.0) as f32,
            2.0,
            RED,
        );
    }
}

impl Add for EucPoint {
    type Output = EucPoint;

    fn add(self, other: EucPoint) -> EucPoint {
        EucPoint {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul<f64> for EucPoint {
    type Output = EucPoint;

    fn mul(self, scalar: f64) -> EucPoint {
        EucPoint {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

pub struct EucLine {
    p1: EucPoint,
    p2: EucPoint,
}

impl EucLine {
    pub fn new(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> Self {
        EucLine {
            p1: EucPoint {
                x: x1,
                y: y1,
                z: z1,
            },
            p2: EucPoint {
                x: x2,
                y: y2,
                z: z2,
            },
        }
    }
}

impl Drawable for EucLine {
    fn draw(&self) {
        let resolution = 200;
        for i in 0..(resolution + 1) {
            let new_point = (self.p1 * i as f64 + self.p2 * ((resolution - i) as f64))
                * (1.0 / resolution as f64);
            new_point.draw();
        }
    }
}

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
}

impl<T: std::fmt::Display + Copy> std::fmt::Display for Vector3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

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

pub struct Triangle {
    p1: Vector3<f32>,
    p2: Vector3<f32>,
    p3: Vector3<f32>,
    normal: Vector3<f32>,
}

impl std::fmt::Display for Triangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
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
}

#[derive(Clone, Copy, PartialEq)]
pub struct SphPoint {
    pub r: f64,
    pub po: Option<f64>,
    pub az: Option<f64>,
}

impl From<EucPoint> for SphPoint {
    fn from(item: EucPoint) -> Self {
        let x = item.x;
        let y = item.y;
        let z = item.z;
        let r = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        let xy = (x.powi(2) + y.powi(2)).sqrt();
        // po
        let po: Option<f64> = if xy != 0.0 || z != 0.0 {
            Some(xy.atan2(z))
        } else {
            None
        };

        // az
        let az: Option<f64> = if x != 0.0 || y != 0.0 {
            Some(y.atan2(x))
        } else {
            None
        };
        SphPoint {
            r: r,
            po: po,
            az: az,
        }
    }
}
