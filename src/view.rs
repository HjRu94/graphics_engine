use crate::{
    geometry::{Orientation, Pose, Triangle, Vector3},
    object::{Mesh, Object},
};
pub struct Camera {
    pose: Pose,
}

impl Camera {
    pub fn new(pose: Pose) -> Self {
        Camera { pose: pose }
    }
    pub fn orientation(&self) -> &Orientation {
        self.pose.orientation()
    }

    pub fn project_point(&self, point: &Vector3<f32>) -> Vector3<f32> {
        let p = point.get_array();
        let pos = self.pose.pos().get_array();
        let orient = self.pose.orientation();
        let v = p - pos;

        let rot = orient.unapply(&v.try_into().expect("Matrix is of wrong dimention"));

        let scalar = 1.0 / rot.x();

        Vector3::new(rot.x(), rot.y() * scalar, rot.z() * scalar)
    }

    pub fn project_triangle(&self, triangle: &Triangle) -> Triangle {
        Triangle::new(
            self.project_point(triangle.p1()),
            self.project_point(triangle.p2()),
            self.project_point(triangle.p3()),
            triangle.normal().clone(),
        )
    }
    pub fn project_mesh(&self, mesh: Mesh) -> Mesh {
        let mut triangles: Vec<Triangle> = vec![];
        for triangle in mesh.iter() {
            triangles.push(self.project_triangle(triangle));
        }
        Mesh::new(triangles)
    }
    pub fn project_object(&self, object: &Object) -> Object {
        let zero_pose_object = object.apply_pose();
        Object::new(
            self.project_mesh(zero_pose_object.mesh().clone()),
            zero_pose_object.pose().clone(),
            zero_pose_object.color(),
        )
    }
}
