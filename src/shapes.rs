use super::*;

/// An example sphere shape.
pub struct Sphere {
    /// The center of the sphere.
    pub center: Point,
    /// The radius of the sphere.
    pub radius: f32
}

impl Shape for Sphere {
    fn vertex_at(&self, u: f32, v: f32) -> Result<Vertex, Error> {
        let normal = self.normal_at(u, v)?;
        let x = self.center[0] + self.radius * normal[0];
        let y = self.center[1] + self.radius * normal[1];
        let z = self.center[2] + self.radius * normal[2];
        return Ok([x, y, z]);
    }

    fn normal_at(&self, u: f32, v: f32) -> Result<Normal, Error> {
        let psi = u * std::f32::consts::PI;
        let phi = 2.0 * v * std::f32::consts::PI;
        let x = psi.sin() * phi.cos();
        let y = psi.sin() * phi.sin();
        let z = psi.cos();
        return Ok([x, y, z]);
    }
    
    fn topology(&self) -> Vec<i32> {
        return vec!(1, 2, -2, -1);
    }
}
