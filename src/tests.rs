use super::*;


fn convert_triangle(native: &Triangle) -> stl_io::Triangle {
    let normal = stl_io::Vertex::new(native.normal);
    let v0 = stl_io::Vertex::new(native.vertices[0]);
    let v1 = stl_io::Vertex::new(native.vertices[1]);
    let v2 = stl_io::Vertex::new(native.vertices[2]);
    stl_io::Triangle { normal, vertices: [v0, v1, v2] }
}

fn write_model(model: Model, path: String) {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    let mesh: Vec<stl_io::Triangle> = model.mesh.iter()
        .map(convert_triangle)
        .collect();
    stl_io::write_stl(&mut file, mesh.iter())
        .unwrap();
}

fn make_cube(size: f32) -> Model {
    let mut mesh = Vec::new();
    for flip in [-1.0, 1.0] {
        for index in 0..3 {
            let v00 = {
                let mut v = [flip * size, -size, -size];
                v.rotate_left(index);
                v
            };
            let v10 = {
                let mut v = [flip * size,  size, -size];
                v.rotate_left(index);
                v
            };
            let v01 = {
                let mut v = [flip * size, -size,  size];
                v.rotate_left(index);
                v
            };
            let v11 = {
                let mut v = [flip * size,  size,  size];
                v.rotate_left(index);
                v
            };
            let normal = {
                let mut v = [flip, 0.0, 0.0];
                v.rotate_left(index);
                v
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
    Model { mesh }
}

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


#[test]
fn cube_example() {
    let cube = make_cube(5.0);
    write_model(cube, "cube.stl".to_string());
}

#[test]
fn sphere_example() {
    let sphere = shapes::Sphere {
        center: [0.0, 0.0, 0.0],
        radius: 1.0
    };
    let model = algos::triangularize(sphere, 100).unwrap();
    write_model(model, "sphere.stl".to_string());
}
