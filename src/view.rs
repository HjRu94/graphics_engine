use core::panic;

use crate::geometry::{Pose, Vector3};
use ndarray::prelude::*;
pub struct Camera {
    pose: Pose,
}

impl Camera {
    pub fn new(pose: Pose) -> Self {
        Camera { pose: pose }
    }

    pub fn project_point(&self, point: &Vector3<f32>) -> Option<Vector3<f32>> {
        let p = point.get_array();
        let pos = self.pose.pos().get_array();
        let fac = self.pose.orientation();
        let v = p - pos;

        // Apply the inverse of the camera rotation on the point
        let roll = -fac.x();
        let pitch = -fac.y();
        let yaw = -fac.z();

        let roll_matrix: Array2<f32> = array![
            [1.0, 0.0, 0.0],
            [0.0, roll.cos(), roll.sin()],
            [0.0, -roll.sin(), roll.cos()],
        ];

        let pitch_matrix: Array2<f32> = array![
            [pitch.cos(), 0.0, -pitch.sin()],
            [0.0, 1.0, 0.0],
            [pitch.sin(), 0.0, pitch.cos()],
        ];

        let yaw_matrix: Array2<f32> = array![
            [yaw.cos(), yaw.sin(), 0.0],
            [-yaw.sin(), yaw.cos(), 0.0],
            [0.0, 0.0, 1.0],
        ];

        let rot = &(roll_matrix.dot(&pitch_matrix.dot(&yaw_matrix))).dot(&v);

        if rot[0] < 1.0 {
            return None;
        }
        Some(
            (rot * (1.0 / rot[0]))
                .try_into()
                .expect("Dimension is wrong"),
        )
    }
}
