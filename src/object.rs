extern crate glium;

use crate::geometry;
use crate::settings;

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

impl<const N: usize> Object<N> {
    fn vertices_data(&self) -> [geometry::Point; N + 1] {
        let data: [geometry::Point; N + 1];

        data[0] = center;

        for index in (0..N) {
            data[index + 1] = self.sphere.points[index];
        }

        data
    }

    fn indices_data(&self) -> [u16; 3 * N] {
        let data: [u16; 3 * N];

        for index in (0..N) {
            data[3 * index + 0] = 0;
            data[3 * index + 1] = index;
            data[3 * index + 2] = index + 1;
        }

        data
    }
}
