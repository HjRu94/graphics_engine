use crate::constants::{AMBIENT_LIGHT, FIELD_OF_VIEW, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::object::Scene;
use crate::view::Camera;
use macroquad::prelude::*;
use ndarray::Array1;

pub fn draw_scene(scene: &Scene, camera: &Camera) {
    for object in scene.iter() {
        let projected_object = object.prepare_render(camera);

        for triangle in projected_object.mesh_iter() {
            let p1 = triangle.p1();
            let p2 = triangle.p2();
            let p3 = triangle.p3();

            if p1.x() < 1.0 || p2.x() < 1.0 || p3.x() < 1.0 {
                continue;
            }

            let normal = triangle.normal().get_array();
            let light_dir: Array1<f32> = camera.orientation().direction().into();
            let intensity = normal.dot(&light_dir).max(0.0);

            let intensity = AMBIENT_LIGHT + intensity * (1.0 - AMBIENT_LIGHT);

            let color = Color::new(
                object.color().r * intensity,
                object.color().g * intensity,
                object.color().b * intensity,
                object.color().a,
            );

            draw_triangle(
                vec2(
                    WINDOW_WIDTH as f32 / 2.0 - p1.y() * FIELD_OF_VIEW,
                    WINDOW_HEIGHT as f32 / 2.0 - p1.z() * FIELD_OF_VIEW,
                ),
                vec2(
                    WINDOW_WIDTH as f32 / 2.0 - p2.y() * FIELD_OF_VIEW,
                    WINDOW_HEIGHT as f32 / 2.0 - p2.z() * FIELD_OF_VIEW,
                ),
                vec2(
                    WINDOW_WIDTH as f32 / 2.0 - p3.y() * FIELD_OF_VIEW,
                    WINDOW_HEIGHT as f32 / 2.0 - p3.z() * FIELD_OF_VIEW,
                ),
                color,
            );
        }
    }
}
