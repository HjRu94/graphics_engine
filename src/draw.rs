use crate::constants::{AMBIENT_LIGHT, FIELD_OF_VIEW, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::object::Scene;
use crate::view::Camera;
use macroquad::prelude::*;
use ndarray::Array1;

use std::time::{Duration, Instant};

pub fn draw_scene(scene: &Scene, camera: &Camera) {
    let total_start = std::time::Instant::now();
    let mut prepare_render_time = std::time::Duration::ZERO;
    let mut triangle_iteration_time = std::time::Duration::ZERO;
    let mut lighting_time = std::time::Duration::ZERO;
    let mut draw_time = std::time::Duration::ZERO;

    for object in scene.iter() {
        let t0 = std::time::Instant::now();

        let projected_object = object.prepare_render(camera);

        prepare_render_time += t0.elapsed();

        for triangle in projected_object.mesh_iter() {
            let t4 = std::time::Instant::now();

            let p1 = triangle.p1();
            let p2 = triangle.p2();
            let p3 = triangle.p3();

            if p1.x() < 1.0 || p2.x() < 1.0 || p3.x() < 1.0 {
                continue;
            }

            triangle_iteration_time += t4.elapsed();

            // Lighting calculation
            let t5 = std::time::Instant::now();

            let normal = triangle.normal().get_array();
            let light_dir: Array1<f32> = camera.orientation().direction().into();
            let intensity = normal.dot(&light_dir).max(0.0);
            let intensity = AMBIENT_LIGHT + intensity * (1.0 - AMBIENT_LIGHT);

            lighting_time += t5.elapsed();

            // Drawing
            let t6 = std::time::Instant::now();

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

            draw_time += t6.elapsed();
        }
    }

    let _total_time = total_start.elapsed();
    #[cfg(feature = "timing")]
    {
        println!("--- Render Timing Report ---");
        println!("Total render time:           {:?}", _total_time);
        println!("  Object preparation time:   {:?}", prepare_render_time);
        println!("  Triangle iteration time:   {:?}", triangle_iteration_time);
        println!("  Lighting computation time: {:?}", lighting_time);
        println!("  Triangle drawing time:     {:?}", draw_time);
        println!("-----------------------------");
    }
}
