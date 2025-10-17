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

    pub fn project_point(&self, point: &Vector3<f32>) -> Vector3<f32> {
        let p = point.get_array();
        let pos = self.pose.pos().get_array();
        let orient = self.pose.orientation();
        let v = p - pos;

        let rot = orient.unapply(v.try_into().expect("Matrix is of wrong dimention"));

        let scalar = 1.0 / rot.x();

        Vector3::new(rot.x(), rot.y() * scalar, rot.z() * scalar)
    }
}
