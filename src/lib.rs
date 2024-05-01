/// Error Types used inside this crate.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// todo: remove later!
    #[error("Generic Error: {0}")]
    Generic(String),
    /// Error for when path distance is too small. 
    #[error("distance {0} >= 0.0 is required")]
    LowerBoundError(f32),
    /// Error for when path distance is too large.
    #[error("distance {0} <= 1.0 is required")]
    UpperBoundError(f32),
    /// Error for when a mesh has too little triangles.
    #[error("requires {requires} triangles but only has {expected}")]
    MinimalTriangleError {
        requires: u32,
        expected: u32
    }
}

/// A vector for a point in 3D space.
pub type Point = [f32; 3];

/// A vector for one mesh vertex.
pub type Vertex = [f32; 3];

/// A vector for one mesh normal.
pub type Normal = [f32; 3];

/// A 1D path embedded in 3D space.
pub trait Path {
    /// Returns the vertex after walking the path
    /// for a distance between 0.0 and 1.0.
    fn vertex_at(&self, distance: f32) -> Result<Vertex, Error>;
    /// Returns one normal after walking the path
    /// for a distance between 0.0 as 1.0.
    fn normal_at(&self, distance: f32) -> Result<Normal, Error>;
}

/// A 2D shape embedded in 3D space.
pub trait Shape {
    /// Returns the vertex after walking on the
    /// surface given by the shape coordinates.
    fn vertex_at(&self, u: f32, v: f32) -> Result<Vertex, Error>;
    /// Returns the normal after walking on the
    /// surface given by the shape coordinates.
    fn normal_at(&self, u: f32, v: f32) -> Result<Normal, Error>;
    /// Returns the fundamental polygon that yields
    /// toplogy of the cluing diagram for the shape.
    fn topology(&self) -> Vec<i32>;
}

/// A triangle storing its normal and vertices.
pub struct Triangle {
    /// The normal of the triangle.
    pub normal: Normal,
    /// The vertices of the triangle.
    pub vertices: [Vertex; 3],
}

/// A model stores the triangulated mesh of shapes.
pub struct Model {
    /// The triangulated mesh of the model.
    pub mesh: Vec<Triangle>
}

pub mod algos;
pub mod paths;
pub mod shapes;

#[cfg(test)]
mod tests;
