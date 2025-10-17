use learning_graphics::draw::draw_scene;
use learning_graphics::geometry::{Pose, Vector3};
use learning_graphics::object::{Mesh, Object, Scene};
use learning_graphics::view::Camera;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Graphics".to_owned(),
        window_width: 2000,
        window_height: 2000,
        high_dpi: true,
        ..Default::default()
    }
}

// FOW pi radiants x pi radiants

#[macroquad::main(window_conf)]
async fn main() {
    let mut z = -0.78;
    let mesh = Mesh::try_from_stl_file("./3d_models/apa.stl").expect("File doesn't exist");

    loop {
        z += 0.03;
        let camera_pos = Vector3::new(-5.0, -5.0, 5.0);
        let camera_facing = Vector3::new(0.0, -0.78, z);
        let camera_pose = Pose::new(camera_pos, camera_facing);
        let camera = Camera::new(camera_pose);

        let pose = Pose::new(Vector3::zero(), Vector3::zero());
        let object = Object::new(mesh.clone(), pose, Color::new(0.5, 0.0, 1.0, 1.0));
        let objects = vec![object];
        let scene = Scene::new(objects);

        clear_background(BLACK);
        draw_scene(&scene, &camera);
        next_frame().await;
    }
}
