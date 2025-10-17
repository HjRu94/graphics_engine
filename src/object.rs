use crate::geometry::{Pose, Triangle, Vector3};
use crate::object;
use macroquad::prelude::Color;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Clone)]
pub struct Mesh(Vec<Triangle>);

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

        let mut triangles: Vec<Triangle> = vec![];

        for _ in 0..n_triangles {
            let normal = read_vector3(&mut reader)?;
            let p1 = read_vector3(&mut reader)?;
            let p2 = read_vector3(&mut reader)?;
            let p3 = read_vector3(&mut reader)?;

            let _end = reader.read_u16::<LittleEndian>()?;

            triangles.push(Triangle::new(p1, p2, p3, normal));
        }

        Ok(Mesh(triangles))
    }
    pub fn iter(&self) -> impl Iterator<Item = &Triangle> {
        self.0.iter()
    }
}

impl std::fmt::Display for Mesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut ret = String::from("");
        for triangle in &self.0 {
            ret += &format!("{}", triangle);
            ret += "\n"
        }
        write!(f, "Object: \n{}", ret)
    }
}

pub struct Object {
    mesh: Mesh,
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
    pub fn color(&self) -> Color {
        self.color
    }
    pub fn mesh_iter(&self) -> impl Iterator<Item = &Triangle> {
        self.mesh.iter()
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
