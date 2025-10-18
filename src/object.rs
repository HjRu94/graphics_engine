use crate::geometry::{Pose, Triangle, Vector3};
use crate::view::Camera;
use macroquad::prelude::Color;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Clone)]
pub struct Mesh {
    vertex_set: Vec<Vector3<f32>>,
    face_set: Vec<(usize, usize, usize)>,
    normal_set: Vec<Vector3<f32>>,
}

fn read_vector3<R: Read>(reader: &mut R) -> std::io::Result<Vector3<f32>> {
    let x = reader.read_f32::<LittleEndian>()?;
    let y = reader.read_f32::<LittleEndian>()?;
    let z = reader.read_f32::<LittleEndian>()?;
    Ok(Vector3::new(x, y, z))
}

impl Mesh {
    pub fn try_from_stl_file(filename: &str) -> std::io::Result<Self> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);

        let mut _header = [0u8; 80];
        reader.read_exact(&mut _header)?;

        let n_triangles = reader.read_u32::<LittleEndian>()?;

        let mut vertexes: Vec<Vector3<f32>> = vec![];
        let mut faces: Vec<(usize, usize, usize)> = vec![];
        let mut normals: Vec<Vector3<f32>> = vec![];
        let mut index_counter: usize = 0;
        for _ in 0..n_triangles {
            normals.push(read_vector3(&mut reader)?);
            for _ in 0..3 {
                vertexes.push(read_vector3(&mut reader)?);
            }

            let _end = reader.read_u16::<LittleEndian>()?;
            faces.push((index_counter, index_counter + 1, index_counter + 2));
            index_counter += 3
        }

        Ok(Mesh {
            vertex_set: vertexes,
            face_set: faces,
            normal_set: normals,
        })
    }
    pub fn alternating_plane(n: i32, square_size: f32, even: bool) -> Self {
        let mut vertexes: Vec<Vector3<f32>> = vec![];
        let mut faces: Vec<(usize, usize, usize)> = vec![];
        let mut normals: Vec<Vector3<f32>> = vec![];
        let mut index_counter: usize = 0;

        for i in (-n)..n {
            for j in (-n)..n {
                if (i + j).rem_euclid(2) == even as i32 {
                    vertexes.push(Vector3::new(
                        i as f32 * square_size,
                        j as f32 * square_size,
                        0.0,
                    ));
                    vertexes.push(Vector3::new(
                        (i + 1) as f32 * square_size,
                        j as f32 * square_size,
                        0.0,
                    ));
                    vertexes.push(Vector3::new(
                        i as f32 * square_size,
                        (j + 1) as f32 * square_size,
                        0.0,
                    ));
                    vertexes.push(Vector3::new(
                        (i + 1) as f32 * square_size,
                        (j + 1) as f32 * square_size,
                        0.0,
                    ));
                    normals.push(Vector3::new(0.0, 0.0, 1.0));
                    normals.push(Vector3::new(0.0, 0.0, 1.0));

                    faces.push((index_counter, index_counter + 1, index_counter + 3));
                    faces.push((index_counter, index_counter + 2, index_counter + 3));
                    index_counter += 4;
                }
            }
        }

        Mesh {
            vertex_set: vertexes,
            face_set: faces,
            normal_set: normals,
        }
    }
    pub fn sort_by_x(&mut self) {
        // Zip faces and normals together
        let mut combined: Vec<_> = self.face_set.iter().zip(self.normal_set.iter()).collect();

        // Sort them together based on the average x-coordinate of each face
        combined.sort_by(|(a_face, _), (b_face, _)| {
            let depth_a = (self.vertex_set[a_face.0].x()
                + self.vertex_set[a_face.1].x()
                + self.vertex_set[a_face.2].x())
                / 3.0;
            let depth_b = (self.vertex_set[b_face.0].x()
                + self.vertex_set[b_face.1].x()
                + self.vertex_set[b_face.2].x())
                / 3.0;

            // Sort from farthest to nearest
            depth_b
                .partial_cmp(&depth_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Unzip them back into separate vectors
        let (faces, normals): (Vec<_>, Vec<_>) =
            combined.into_iter().map(|(f, n)| (*f, n.clone())).unzip();

        self.face_set = faces;
        self.normal_set = normals;
    }
    pub fn remove_away_faceing_triangles(&mut self, camera: &Camera) {
        (self.face_set, self.normal_set) = self
            .face_set
            .iter()
            .zip(self.normal_set.iter())
            .filter(|(_a, b)| camera.orientation().unapply(b).x() < 0.0)
            .map(|(a, b)| (*a, b.clone()))
            .unzip()
    }
    pub fn reverse(&mut self) {
        self.face_set.reverse();
        self.normal_set.reverse();
    }
    pub fn apply_pose(&mut self, pose: &Pose) {
        for vertex in self.vertex_set.iter_mut() {
            vertex.apply_pose(pose);
        }
        for normal in self.normal_set.iter_mut() {
            *normal = pose.orientation().apply(normal);
        }
    }

    pub fn triangle_set(&self) -> Vec<Triangle> {
        let mut triangles: Vec<Triangle> = vec![];
        for ((v1, v2, v3), normal) in self.face_set.iter().zip(self.normal_set.iter()) {
            triangles.push(Triangle::new(
                self.vertex_set[*v1].clone(),
                self.vertex_set[*v2].clone(),
                self.vertex_set[*v3].clone(),
                normal.clone(),
            ))
        }
        triangles
    }
    pub fn camera_project(&mut self, camera: &Camera) {
        for vertex in self.vertex_set.iter_mut() {
            vertex.camera_project(camera);
        }
    }
}

#[derive(Clone)]
pub struct Object {
    pub mesh: Mesh,
    pose: Pose,
    color: Color,
}

impl Object {
    pub fn new(mesh: Mesh, pose: Pose, color: Color) -> Self {
        Object {
            mesh: mesh,
            pose: pose,
            color: color,
        }
    }
    pub fn apply_pose(&mut self) {
        self.mesh.apply_pose(&self.pose);
    }
    pub fn prepare_render(&mut self, camera: &Camera) {
        self.camera_project(camera);
        self.mesh.remove_away_faceing_triangles(camera);
        self.mesh.sort_by_x();
    }
    pub fn reverse(&mut self) {
        self.mesh.reverse();
    }
    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    pub fn pose(&self) -> &Pose {
        &self.pose
    }
    pub fn color(&self) -> Color {
        self.color
    }
    pub fn triangle_set(&self) -> Vec<Triangle> {
        self.mesh.triangle_set()
    }
    pub fn camera_project(&mut self, camera: &Camera) {
        self.apply_pose();
        self.mesh.camera_project(camera);
    }
}

pub struct Scene {
    objects: Vec<Object>,
}

impl Scene {
    pub const EMPTY: Scene = Scene { objects: vec![] };
    pub fn new(objects: Vec<Object>) -> Self {
        Scene { objects: objects }
    }
    pub fn iter(&self) -> impl Iterator<Item = &Object> {
        self.objects.iter()
    }
}
