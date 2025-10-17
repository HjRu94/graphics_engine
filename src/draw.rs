use crate::geometry::Vector3;
use crate::object::{self, Scene};
use crate::view::Camera;
use macroquad::prelude::*;

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

            if p1_r == None || p2_r == None || p3_r == None {
                continue;
            }
            let p1_r = p1_r.unwrap();
            let p2_r = p2_r.unwrap();
            let p3_r = p3_r.unwrap();

            draw_triangle(
                vec2(500.0 - p1_r.y() * scalar, 500.0 - p1_r.z() * scalar),
                vec2(500.0 - p2_r.y() * scalar, 500.0 - p2_r.z() * scalar),
                vec2(500.0 - p3_r.y() * scalar, 500.0 - p3_r.z() * scalar),
                object.color(),
            );
        }
    }
}
