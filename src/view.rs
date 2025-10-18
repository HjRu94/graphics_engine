use crate::geometry::{Orientation, Pose, Vector3};
use macroquad::prelude::*;
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

pub struct Facing {
    facing_point: Vector3<f32>,
    camera_orientation: Orientation,
    camera_distance: f32,
    last_mouse_pos: Option<Vec2>, // For drag tracking
}

impl Facing {
    pub fn new(
        facing_point: Vector3<f32>,
        camera_orientation: Orientation,
        camera_distance: f32,
    ) -> Self {
        Facing {
            facing_point,
            camera_orientation,
            camera_distance,
            last_mouse_pos: None,
        }
    }

    pub fn generate_camera(&self) -> Camera {
        let pos: Vector3<f32> =
            self.camera_orientation.direction() * self.camera_distance + self.facing_point.clone();
        Camera::new(Pose::new(pos, self.camera_orientation.clone()))
    }

    pub fn update_camera_distance(&mut self) {
        let (_scroll_x, scroll_y) = mouse_wheel();
        if scroll_y < 0.0 {
            self.camera_distance *= 1.1;
        } else if scroll_y > 0.0 {
            self.camera_distance *= 0.9;
        }
    }

    /// Moves the facing point when right mouse is dragged.
    pub fn drag_camera_pos(&mut self) {
        // Convert 2D mouse delta into 3D movement in camera's local plane
        let mouse_pos = mouse_position();
        let mouse_vec = vec2(mouse_pos.0, mouse_pos.1);

        if is_mouse_button_down(MouseButton::Right) {
            if let Some(last_pos) = self.last_mouse_pos {
                let delta = mouse_vec - last_pos;

                let sensitivity = 0.005;

                self.camera_orientation = Orientation::new(
                    self.camera_orientation.roll(),
                    self.camera_orientation.pitch() - sensitivity * delta.y,
                    self.camera_orientation.yaw() + sensitivity * delta.x,
                );
            }
            self.last_mouse_pos = Some(mouse_vec);
        } else if is_mouse_button_down(MouseButton::Left) {
            if let Some(last_pos) = self.last_mouse_pos {
                let delta = mouse_vec - last_pos;

                let sensitivity = 0.002 * self.camera_distance;

                let mut movement = Vector3::new(0.0, sensitivity * delta.x, sensitivity * delta.y);

                movement = self.camera_orientation.apply(&movement);

                self.facing_point = self.facing_point.clone() + movement;
            }
            self.last_mouse_pos = Some(mouse_vec);
        } else {
            // Reset when not dragging
            self.last_mouse_pos = None;
        }
    }
}
