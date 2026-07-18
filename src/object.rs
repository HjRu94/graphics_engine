use crate::geometry::{Pose, Triangle, Vector3};
use crate::view::Camera;
use core::f32;
use macroquad::prelude::Color;
use ndarray_linalg::norm;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::{Error, ErrorKind};

use std::collections::HashMap;

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
    pub fn try_new(
        vertex_set: Vec<Vector3<f32>>,
        face_set: Vec<(usize, usize, usize)>,
        normal_set: Vec<Vector3<f32>>,
    ) -> std::io::Result<Self> {
        if face_set.len() != normal_set.len() {
            return Err(Error::other(
                "face set and normal set have different length",
            ));
        }
        let n_vertexes = vertex_set.len();
        for (v1, v2, v3) in &face_set {
            for i in [v1, v2, v3] {
                if *i >= n_vertexes {
                    return Err(Error::other(
                        "face set references vertesies that don't exist",
                    ));
                }
            }
        }
        Ok(Mesh {
            vertex_set: vertex_set,
            face_set: face_set,
            normal_set: normal_set,
        })
    }
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
        let mut ret = Mesh {
            vertex_set: vertexes,
            face_set: faces,
            normal_set: normals,
        };

        ret.optimize();

        Ok(ret)
    }
    pub fn optimize(&mut self) {
        let mut unique_vertices: Vec<Vector3<f32>> = Vec::new();
        let mut vertex_map: HashMap<Vector3<f32>, usize> = HashMap::new();

        // Map old indices → new indices
        let mut remap: Vec<usize> = vec![0; self.vertex_set.len()];

        for (old_idx, v) in self.vertex_set.iter().enumerate() {
            if let Some(&new_idx) = vertex_map.get(v) {
                remap[old_idx] = new_idx;
            } else {
                let new_idx = unique_vertices.len();
                vertex_map.insert(v.clone(), new_idx);
                unique_vertices.push(v.clone());
                remap[old_idx] = new_idx;
            }
        }

        // Update faces using new indices
        for face in &mut self.face_set {
            face.0 = remap[face.0];
            face.1 = remap[face.1];
            face.2 = remap[face.2];
        }

        self.vertex_set = unique_vertices;
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

        let mut ret = Mesh {
            vertex_set: vertexes,
            face_set: faces,
            normal_set: normals,
        };
        ret.optimize();
        ret
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

    pub fn n_faces(&self) -> usize {
        self.face_set.len()
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
pub struct ColorMap {
    colors: Vec<Color>,
}

#[derive(Clone)]
pub enum MeshColor {
    SolidColor(Color),
    VertexColorMap(ColorMap),
}

impl MeshColor {
    fn gradient(color1: Color, color2: Color, mesh: &Mesh) -> Self {
        let mut max_x: f32 = f32::NEG_INFINITY;
        let mut min_x: f32 = f32::INFINITY;

        for vertex in &mesh.vertex_set {
            max_x = max_x.max(vertex.x());
            min_x = min_x.min(vertex.x());
        }
        let mut colors: Vec<Color> = vec![];

        for vertex in &mesh.vertex_set {
            let ratio = (vertex.x() - min_x) / (max_x - min_x);

            let r = color1.r * ratio + color2.r * (1.0 - ratio);
            let g = color1.g * ratio + color2.g * (1.0 - ratio);
            let b = color1.b * ratio + color2.b * (1.0 - ratio);

            colors.push(Color::new(r, g, b, 1.0))
        }
        MeshColor::VertexColorMap(ColorMap { colors: colors })
    }
}

#[derive(Clone)]
pub struct Object {
    pub mesh: Mesh,
    pose: Pose,
    color: MeshColor,
}

impl Object {
    pub fn new(mesh: Mesh, pose: Pose, color: Color) -> Self {
        Object {
            mesh: mesh,
            pose: pose,
            color: MeshColor::SolidColor(color),
        }
    }

    pub fn new_gradient_object(mesh: Mesh, pose: Pose, color1: Color, color2: Color) -> Self {
        let gradient = MeshColor::gradient(color1, color2, &mesh);
        Object {
            mesh: mesh,
            pose: pose,
            color: gradient,
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
    pub fn color(&self) -> &MeshColor {
        &self.color
    }
    pub fn triangle_set(&self) -> Vec<Triangle> {
        self.mesh.triangle_set()
    }
    pub fn triangle_color_set(&self) -> Vec<(Triangle, Color, Color, Color)> {
        let mut color_triangles: Vec<(Triangle, Color, Color, Color)> = vec![];
        for i in 0..self.mesh.n_faces() {
            let v1 = self.mesh.face_set[i].0;
            let v2 = self.mesh.face_set[i].1;
            let v3 = self.mesh.face_set[i].2;
            let normal = self.mesh.normal_set[i].clone();
            let triangle = Triangle::new(
                self.mesh.vertex_set[v1].clone(),
                self.mesh.vertex_set[v2].clone(),
                self.mesh.vertex_set[v3].clone(),
                normal.clone(),
            );
            let (color1, color2, color3) = match self.color.clone() {
                MeshColor::SolidColor(color) => (color, color, color),
                MeshColor::VertexColorMap(color_map) => (
                    color_map.colors[v1],
                    color_map.colors[v2],
                    color_map.colors[v3],
                ),
            };
            color_triangles.push((triangle, color1, color2, color3));
        }
        color_triangles
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
    pub fn plane_world(
        plane_size: i32,
        plane_square_size: f32,
        color1: Color,
        color2: Color,
    ) -> Self {
        //even plane
        let plane_mesh_even = Mesh::alternating_plane(plane_size, plane_square_size, true);
        let plane_object_even = Object::new(plane_mesh_even, Pose::zero(), color1);

        //odd plane
        let plane_mesh_odd = Mesh::alternating_plane(plane_size, plane_square_size, false);
        let plane_object_odd = Object::new(plane_mesh_odd, Pose::zero(), color2);

        let objects = vec![plane_object_even, plane_object_odd];
        Scene { objects }
    }
    pub fn push_object(&mut self, object: Object) {
        self.objects.push(object);
    }
    pub fn iter(&self) -> impl Iterator<Item = &Object> {
        self.objects.iter()
    }
}
