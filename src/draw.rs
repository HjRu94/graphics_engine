use crate::geometry::Vector3;
use crate::object::{self, Scene};
use crate::view::Camera;
use macroquad::prelude::*;
use ndarray::Array1;

pub fn draw_scene(scene: &Scene, camera: &Camera) {
    let scalar = 200.0;
    for object in scene.iter() {
        for triangle in object.mesh_iter() {
            let p1 = triangle.p1();
            let p2 = triangle.p2();
            let p3 = triangle.p3();

            let p1_r = camera.project_point(p1);
            let p2_r = camera.project_point(p2);
            let p3_r = camera.project_point(p3);

            if p1_r.x() < 1.0 || p2_r.x() < 1.0 || p3_r.x() < 1.0 {
                continue;
            }

            let normal = triangle.normal().get_array();
            let light_dir = Array1::from(vec![0.0, 0.0, -1.0]);
            let intensity = normal.dot(&light_dir).max(0.0);

            let ambient = 0.5;
            let intensity = ambient + intensity * (1.0 - ambient);

            let color = Color::new(
                object.color().r * intensity,
                object.color().g * intensity,
                object.color().b * intensity,
                object.color().a,
            );

            draw_triangle(
                vec2(500.0 - p1_r.y() * scalar, 500.0 - p1_r.z() * scalar),
                vec2(500.0 - p2_r.y() * scalar, 500.0 - p2_r.z() * scalar),
                vec2(500.0 - p3_r.y() * scalar, 500.0 - p3_r.z() * scalar),
                color,
            );
        }
    }
}
