extern crate glium;

use crate::geometry;
use crate::settings;

/// Generate vertices array to vertex buffer
///
/// * `sphere`: Sphere to get verices array
fn vertices_data(sphere: geometry::Sphere) -> [geometry::Point; 1 + settings::LENGTH] {
    let mut data = [geometry::Point::default(); 1 + settings::LENGTH];

    data[0].color = sphere.points[0].color;

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
/// * `vertices`: Vertex buffer
/// * `indices`: Index buffer
/// * `program`: Shader program
pub struct Object {
    pub sphere: geometry::Sphere,
    pub vertices: glium::VertexBuffer<geometry::Point>,
    pub indices: glium::IndexBuffer<u16>,
    pub program: glium::Program,
}

impl Object {
    /// Create a new object
    ///
    /// * `center`: Object center
    /// * `radius`: Object radius
    /// * `display`: Display
    pub fn new(center: [f32; 2], radius: f32, display: &glium::Display) -> Object {
        let sphere = geometry::Sphere::new(center, radius);

        let shape = vertices_data(sphere);
        let index = indices_data();

        let vertex = r#"
            #version 330

            uniform vec2 center;

            in vec2 position;
            in vec3 color;

            out vec3 _color;

            void main() {
                gl_Position = vec4(position + center, 0.0, 1.0);
                _color = color;
            }
        "#;

        let fragment = r#"
            #version 330
            
            in vec3 _color;

            out vec4 color;

            void main() {
                color = vec4(_color, 1.0);
            }
        "#;

        let program = glium::Program::from_source(display, vertex, fragment, None).unwrap();

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

    /// Update position and velocity of object sphere
    ///
    /// * `center`: Object sphere center
    /// * `velocity`: Object sphere velocity
    pub fn update(&mut self, center: [f32; 2], velocity: [f32; 2]) {
        self.sphere.center = center;
        self.sphere.velocity = velocity;
    }

    /// Render object
    ///
    /// * `display`: Display where render
    pub fn render(&mut self, frame: &mut glium::Frame, parameters: glium::DrawParameters) {
        use glium::Surface;

        let uniforms = glium::uniform! {
            center: self.sphere.center,
        };

        frame
            .draw(
                &self.vertices,
                &self.indices,
                &self.program,
                &uniforms,
                &parameters,
            )
            .unwrap();
    }
}
