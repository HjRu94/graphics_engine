use crate::object::Scene;
use crate::view::Camera;
use macroquad::prelude::*;
use ndarray::Array1;

pub fn draw_scene(scene: &Scene, camera: &Camera) {
    let scalar = 200.0;
    for object in scene.iter() {
        let mut projected_object = camera.project_object(object);
        projected_object.sort_by_x();
        projected_object.reverse();
        for triangle in projected_object.mesh_iter() {
            let p1 = triangle.p1();
            let p2 = triangle.p2();
            let p3 = triangle.p3();

            if p1.x() < 1.0 || p2.x() < 1.0 || p3.x() < 1.0 {
                continue;
            }

            let normal = triangle.normal().get_array();
            let light_dir = Array1::from(vec![0.0, 0.0, 1.0]);
            //let light_dir: Array1<f32> = camera.orientation().direction().into();
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
                vec2(500.0 - p1.y() * scalar, 500.0 - p1.z() * scalar),
                vec2(500.0 - p2.y() * scalar, 500.0 - p2.z() * scalar),
                vec2(500.0 - p3.y() * scalar, 500.0 - p3.z() * scalar),
                color,
            );
        }
    }
}
