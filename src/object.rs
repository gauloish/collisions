extern crate glium;

use glium::Surface;

use crate::geometry;
use crate::settings;

/// Generate vertices array to vertex buffer
///
/// * `sphere`: Sphere to get verices array
fn vertices_data(
    sphere: &geometry::Sphere,
    color: usize,
) -> [geometry::Point; 1 + settings::LENGTH] {
    let mut data = [geometry::Point::from(color); 1 + settings::LENGTH];

    data[0] = sphere.center;

    for index in 0..settings::LENGTH {
        data[index + 1] = sphere.points[index];
    }

    data
}

/// Generate indices array to index buffer
fn indices_data() -> [u16; 3 * settings::LENGTH] {
    let mut data = [0; 3 * settings::LENGTH];

    for index in 0..settings::LENGTH {
        data[3 * index + 0] = 0;
        data[3 * index + 1] = (index as u16) + 1;
        data[3 * index + 2] = (index as u16) + 2;
    }

    data[3 * settings::LENGTH - 1] = 1;

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
pub struct Object {
    sphere: geometry::Sphere,
    vertices: glium::VertexBuffer<geometry::Point>,
    indices: glium::IndexBuffer<u16>,
    program: glium::Program,
}

impl Object {
    /// Create a new object
    ///
    /// * `color`: Object color
    /// * `radius`: Object radius
    /// * `display`: Display
    pub fn new(color: usize, radius: f64, display: &glium::Display) -> Object {
        let center: geometry::Point = geometry::Point::from(color);
        let sphere: geometry::Sphere = geometry::Sphere::new(center, radius, color);

        let shape = vertices_data(&sphere, color);
        let index = indices_data();

        let vertex_shader = r#"
            #version 140

            in vec2 position;
            in vec3 color;

            out vec3 _color;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
                _color = color;
            }
        "#;

        let fragment_shader = r#"
            #version 140
            
            in vec3 _color;

            out vec4 color;

            void main() {
                color = vec4(_color, 0.0);
            }
        "#;

        let program =
            glium::Program::from_source(display, vertex_shader, fragment_shader, None).unwrap();

        let vertices = glium::VertexBuffer::new(display, &shape).unwrap();
        let indices =
            glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &index)
                .unwrap();

        Object {
            sphere,
            vertices,
            indices,
            program,
        }
    }

    /// Render object
    ///
    /// * `display`: Display where render
    pub fn render(&self, display: &glium::Display) {
        let mut frame = display.draw();

        frame.clear_color(0.96, 0.96, 0.96, 1.0);
        frame
            .draw(
                &self.vertices,
                &self.indices,
                &self.program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        frame.finish().unwrap();
    }
}
