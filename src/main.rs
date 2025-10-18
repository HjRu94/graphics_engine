use learning_graphics::draw::draw_scene;
use learning_graphics::geometry::{Orientation, Pose, Vector3};
use learning_graphics::object::{self, Mesh, Object, Scene};
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
    let mesh = Mesh::try_from_stl_file("./3d_models/apa.stl").expect("File doesn't exist");

    //even plane
    let plane_mesh_even = Mesh::alternating_plane(10, 2.0, true);
    let plane_object_even = Object::new(
        plane_mesh_even,
        Pose::zero(),
        Color::new(1.0, 1.0, 1.0, 1.0),
    );

    //odd plane
    let plane_mesh_odd = Mesh::alternating_plane(10, 2.0, false);
    let plane_object_odd =
        Object::new(plane_mesh_odd, Pose::zero(), Color::new(0.2, 0.2, 0.2, 1.0));

    let object_orientation = Orientation::new(x, 0.0, 0.0);

    let pose = Pose::new(Vector3::zero(), object_orientation);
    let object = Object::new(mesh.clone(), pose, Color::new(0.5, 0.0, 1.0, 1.0));
    let objects = vec![plane_object_even, plane_object_odd, object];
    let scene = Scene::new(objects);
    loop {
        z += 0.03;
        x += 0.03;
        let camera_pos = Vector3::new(
            6.3 * (3.14 as f32 - z).cos(),
            6.3 * (3.14 as f32 - z).sin(),
            6.3,
        );
        let camera_facing = Orientation::new(0.0, -0.78, z);
        let camera_pose = Pose::new(camera_pos, camera_facing);
        let camera = Camera::new(camera_pose);

        clear_background(BLACK);
        draw_scene(&scene, &camera);
        next_frame().await;
    }
}
