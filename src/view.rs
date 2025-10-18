use crate::{
    geometry::{Orientation, Pose, Triangle, Vector3},
    object::{Mesh, Object},
};
pub struct Camera {
    pose: Pose,
}

impl Camera {
    pub fn new(pose: Pose) -> Self {
        Camera { pose: pose }
    }
    pub fn orientation(&self) -> &Orientation {
        self.pose.orientation()
    }
    pub fn pos(&self) -> &Vector3<f32> {
        self.pose.pos()
    }
}

impl std::fmt::Display for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Camera: {}", self.pose)
    }
}
