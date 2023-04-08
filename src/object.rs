extern crate glium;

use crate::geometry;
use crate::settings;

pub struct Object<const N: usize> {
    sphere: geometry::Sphere<N>,
    center: geometry::Point,
    vertices: glium::VertexBuffer,
    indices: glium::IndexBuffer,
    program: glium::Program,
    color: usize,
}
