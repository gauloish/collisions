extern crate glium;

use crate::geometry;
use crate::settings;

/// Generate vertices array to vertex buffer
fn vertices_data<const N: usize>(object: &Object<N>) -> [geometry::Point; N + 1] {
    let data: [geometry::Point; N + 1];

    data[0] = object.center;

    for index in (0..N) {
        data[index + 1] = object.sphere.points[index];
    }

    data
}

/// Generate indices array to index buffer
fn indices_data() -> [u16; 3 * N] {
    let data: [u16; 3 * N];

    for index in (0..N) {
        data[3 * index + 0] = 0;
        data[3 * index + 1] = index;
        data[3 * index + 2] = index + 1;
    }

    data
}

/// A renderizable sphere
///
/// * `sphere`: Object sphere
/// * `center`: Sphere center
/// * `vertices`: Vertex buffer
/// * `indices`: Index buffer
/// * `program`: Shader program
/// * `color`: Index color
pub struct Object<const N: usize> {
    sphere: geometry::Sphere<N>,
    center: geometry::Point,
    vertices: glium::VertexBuffer,
    indices: glium::IndexBuffer,
    program: glium::Program,
    color: usize,
}

impl<const N: usize> Object<N> {}
