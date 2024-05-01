use stl_io::{Normal, Vertex, Triangle};


fn make_cube(size: f32) -> Vec<Triangle> {
    let mut mesh = Vec::new();
    for flip in [-1.0, 1.0] {
        for index in 0..3 {
            let v00 = {
                let mut v = [flip * size, -size, -size];
                v.rotate_left(index);
                Vertex::new(v)
            };
            let v10 = {
                let mut v = [flip * size,  size, -size];
                v.rotate_left(index);
                Vertex::new(v)
            };
            let v01 = {
                let mut v = [flip * size, -size,  size];
                v.rotate_left(index);
                Vertex::new(v)
            };
            let v11 = {
                let mut v = [flip * size,  size,  size];
                v.rotate_left(index);
                Vertex::new(v)
            };
            let normal = {
                let mut v = [flip, 0.0, 0.0];
                v.rotate_left(index);
                Normal::new(v)
            };
            mesh.push(Triangle {
                normal,
                vertices: [v00, v01, v10]
            });
            mesh.push(Triangle {
                normal,
                vertices: [v11, v10, v01]
            });
        }
    }
    mesh
}

/// foo
#[test]
fn usage_example() {
    let mesh = make_cube(5.0);
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("mesh.stl")
        .unwrap();
    stl_io::write_stl(&mut file, mesh.iter())
        .unwrap();
    /* This would be the goal, we allow the definition
     * of paths through 3D space, e.g. Circle. Then we
     * can combine multiple paths to get shapes, e.g.
     * by tracing another path. Finally, these shapes
     * can be triangulated to get an actual model with
     * the triangle vertices and their normals.
     */
    // let outer_circle = Circle {
    //     center: Point::new(0, 0, 0),
    //     normal: Point::new(0, 1, 0),
    //     radius: 3,
    // };
    // let inner_circle = Circle {
    //     center: Point::new(3, 0, 0),
    //     normal: Point::new(0, 0, 1),
    //     radius: 1
    // };
    // let torus = inner_circle.trace(outer_circle);
    // let torus = torus.lookAt(0, 0, 1);
    // let model = torus.triangulate();
    // Store model in some file format or use it ...
}