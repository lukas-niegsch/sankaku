use super::*;

/// A path obtained of chaining two other paths.
pub struct ChainPath<A: Path, B: Path> {
    fst_path: A,
    snd_path: B
}

/// We chain paths by walking either path twice as fast.
impl<A: Path, B: Path> Path for ChainPath<A, B> {
    fn vertex_at(&self, distance: f32) -> Result<Vertex, Error> {
        if distance <= 0.5 {
            self.fst_path.vertex_at(2.0 * distance)
        } else {
            self.snd_path.vertex_at(2.0 * distance)
        }
    }

    fn normal_at(&self, distance: f32) -> Result<Normal, Error> {
        if distance <= 0.5 {
            self.fst_path.normal_at(2.0 * distance)
        } else {
            self.snd_path.normal_at(2.0 * distance)
        }
    }
}