use graphics_engine::constants::{
    MONITOR_SCALING, PLANE_DARK_COLOR, PLANE_LIGHT_COLOR, PLANE_SIZE, PLANE_SQUARE_SIZE,
    WINDOW_HEIGHT, WINDOW_WIDTH,
};
use graphics_engine::draw::draw_scene;
use graphics_engine::geometry::{Orientation, Pose, Vector3};
use graphics_engine::object::{Mesh, Object, Scene};
use graphics_engine::view::Facing;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Graphics".to_owned(),
        window_width: (WINDOW_WIDTH as f32 * MONITOR_SCALING).round() as i32,
        window_height: (WINDOW_HEIGHT as f32 * MONITOR_SCALING).round() as i32,
        high_dpi: true,
        ..Default::default()
    }
}

// FOW pi radiants x pi radiants

#[macroquad::main(window_conf)]
async fn main() {
    let mesh = Mesh::try_from_stl_file("./sphere.stl").expect("File doesn't exist");

    let mut scene = Scene::plane_world(
        PLANE_SIZE,
        PLANE_SQUARE_SIZE,
        PLANE_DARK_COLOR,
        PLANE_LIGHT_COLOR,
    );

    let object_orientation = Orientation::new(0.0, 0.0, 0.0);

    let pose = Pose::new(Vector3::zero(), object_orientation);
    let object = Object::new(mesh.clone(), pose, Color::new(0.5, 0.0, 1.0, 1.0));

    let camera_facing = Orientation::new(0.0, -0.78, -0.78);
    let mut facing = Facing::new(Vector3::new(0.0, 0.0, 0.0), camera_facing, 8.0);
    scene.push_object(object);
    loop {
        let camera = facing.generate_camera();

        clear_background(BLACK);
        facing.update_camera_distance();
        facing.drag_camera_pos();
        draw_scene(&scene, &camera);
        draw_fps();
        next_frame().await;
    }
}
