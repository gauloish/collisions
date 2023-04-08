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
