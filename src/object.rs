extern crate glium;

use crate::geometry;
use crate::settings;

/// Generate vertices array to vertex buffer
///
/// * `sphere`: Sphere to get verices array
fn vertices_data<const N: usize>(sphere: &geometry::Sphere<N>) -> [geometry::Point; N + 1] {
    let data: [geometry::Point; N + 1];

    data[0] = sphere.center;

    for index in (0..N) {
        data[index + 1] = sphere.points[index];
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
    vertices: glium::VertexBuffer,
    indices: glium::IndexBuffer,
    program: glium::Program,
    color: usize,
}

impl<const N: usize> Object<N> {
    pub fn new(color: usize, radius: f64, display: &glium::Display) -> Object<N> {
        let shape = vertices_data(&sphere);
        let index = indices_data();

        let vertex_shader = r#"
            #version 140
            in vec2 position;
            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader = r#"
            #version 140
            out vec4 color;
            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        let center: geometry::Point = geometry::Point::from([0.0, 0.0]);
        let sphere: geometry::Sphere<N> = geometry::Sphere::new(center, radius);

        let program =
            glium::Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();

        let vertices = glium::VertexBuffer::new(&display, &shape).unwrap();
        let indices = glium::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        )
        .unwrap();

        Object::<N> {
            sphere,
            vertices,
            indices,
            program,
            color,
        }
    }
}
