use learning_graphics::draw::draw_scene;
use learning_graphics::geometry::{Orientation, Pose, Vector3};
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
    let mut z: f32 = -0.78;
    let mut x: f32 = 0.00;
    let mesh = Mesh::try_from_stl_file("./3d_models/cube.stl").expect("File doesn't exist");

    loop {
        z += 0.03;
        //x += 0.03;
        let camera_pos = Vector3::new(
            2.3 * (3.14 as f32 - z).cos(),
            2.3 * (3.14 as f32 - z).sin(),
            2.3,
        );
        let camera_facing = Orientation::new(0.0, -0.78, z);
        let camera_pose = Pose::new(camera_pos, camera_facing);
        let camera = Camera::new(camera_pose);

        let object_orientation = Orientation::new(x, 0.0, 0.0);
        let pose = Pose::new(Vector3::zero(), object_orientation);
        let object = Object::new(mesh.clone(), pose, Color::new(0.5, 0.0, 1.0, 1.0));
        let objects = vec![object];
        let scene = Scene::new(objects);

        clear_background(BLACK);
        draw_scene(&scene, &camera);
        next_frame().await;
    }
}
