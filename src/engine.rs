extern crate glium;

use crate::geometry::Point;
use crate::object;
use crate::operations;
use crate::settings;

use rand;

/// Generate objects
///
/// * `display`: Display
pub fn generate(display: &glium::backend::glutin::Display) -> Vec<object::Object> {
    let mut objects: Vec<object::Object> = Vec::new();

    let seed = rand::random::<usize>();

    let amount = (8 + seed % (32 - 8)) as f32;

    let length = 1.0 / amount;
    let radius = length * ((20 + seed % 40) as f32) / 100.0;
    let padding = (2.0 - 2.0 * radius * amount) / (amount + 1.0);

    for line in 0..(amount as usize) {
        for column in 0..(amount as usize) {
            let center = [
                (padding + 2.0 * radius) * ((column as f32) + 1.0) - 1.0 - radius,
                (padding + 2.0 * radius) * ((line as f32) + 1.0) - 1.0 - radius,
            ];

            objects.push(object::Object::new(center, radius, display));
        }
    }

    objects
}

/// Reshape viewport when window is resized
///
/// * `display`: Display
pub fn reshape(display: &glium::Display) -> Option<glium::Rect> {
    let window = display.gl_window();

    let mut width = window.window().inner_size().width;
    let mut height = window.window().inner_size().height;

    let mut left: u32 = 0;
    let mut bottom: u32 = 0;

    if width > height {
        left = (width - height) / 2;

        width = height;
    } else if width < height {
        bottom = (height - width) / 2;

        height = width;
    }

    Some(glium::Rect {
        left,
        bottom,
        width,
        height,
    })
}

/// Update sphere positions based in your velocities
///
/// * `objects`: Vector with objects in scene
pub fn update(objects: &mut Vec<object::Object>) {
    for index in 0..objects.len() {
        let mut center = objects[index].sphere.center;
        let velocity = objects[index].sphere.velocity;

        center = operations::add(center, velocity);

        objects[index].update(center, velocity);
    }
}

/// Update sphere velocity based in wall collisions
///
/// * `object`: Object
pub fn wall(object: &mut object::Object) {
    let center = object.sphere.center;
    let radius = object.sphere.radius;

    let mut velocity = object.sphere.velocity;

    for index in 0..2 {
        if center[index].abs() + radius > 1.0 {
            if !(velocity[index].signum() != center[index].signum()) {
                velocity[index] *= -1.0;
            }
        }
    }

    object.update(center, velocity);
}

/// Update spheres velocity based in inner collisions
///
/// * `objects`: Vector of objects
/// * `first`: Index to first object
/// * `second`: Index to second object
pub fn inner(objects: &mut Vec<object::Object>, first: usize, second: usize) {
    let radius = objects[first].sphere.radius;

    let center = [
        objects[first].sphere.center,  //
        objects[second].sphere.center, //
    ];

    let velocity = [
        objects[first].sphere.velocity,  //
        objects[second].sphere.velocity, //
    ];

    let next = [
        operations::add(center[0], velocity[0]), //
        operations::add(center[1], velocity[1]), //
    ];

    for index in 0..2 {
        if operations::dist(next[index], center[(index + 1) % 2]) > 2.0 * radius {
            return;
        }
    }

    objects[first].sphere.velocity = operations::sub(
        velocity[0],
        operations::scale(
            operations::dot(
                operations::sub(velocity[0], velocity[1]),
                operations::sub(center[0], center[1]),
            ) / operations::dist(center[0], center[1]).powf(2.0),
            operations::sub(center[0], center[1]),
        ),
    );

    objects[second].sphere.velocity = operations::sub(
        velocity[1],
        operations::scale(
            operations::dot(
                operations::sub(velocity[1], velocity[0]),
                operations::sub(center[1], center[0]),
            ) / operations::dist(center[1], center[0]).powf(2.0),
            operations::sub(center[1], center[0]),
        ),
    );
}

/// Update sphere velocities based in your collisions
///
/// * `objects`: Vector with objects in scene
pub fn collisions(objects: &mut Vec<object::Object>) {
    for first in 0..objects.len() {
        wall(&mut objects[first]);

        for second in 0..objects.len() {
            if first != second {
                inner(objects, first, second);
            }
        }
    }
}

/// Adjust contact between spheres to avoid intersection
///
/// * `objects`: Vector with objects in scene
pub fn adjust(objects: &mut Vec<object::Object>) {
    let radius = objects[0].sphere.radius;

    for first in 0..objects.len() {
        for second in 0..objects.len() {
            if first == second {
                continue;
            }

            let center = [
                objects[first].sphere.center,  //
                objects[second].sphere.center, //
            ];

            let vector = operations::sub(center[0], center[1]);
            let magnitude = operations::dist(center[0], center[1]);

            let detour = 2.0 * radius - magnitude;

            if detour > 0.0 {
                let correction = operations::scale(0.5 * detour / magnitude, vector);

                objects[first].sphere.center = operations::add(center[0], correction);
                objects[second].sphere.center = operations::sub(center[1], correction);
            }
        }
    }
}

/// Generate draw parameters
///
/// * `display`: Display
pub fn draw(display: &glium::Display) -> glium::DrawParameters {
    let mut parameters = glium::DrawParameters::default();

    parameters.viewport = reshape(display);

    parameters
}

/// Run simulation
pub fn run() {
    use glium::Surface;

    glium::implement_vertex!(Point, position, color);

    let events = glium::glutin::event_loop::EventLoop::new();

    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(false)
        .with_multisampling(8);

    let window = glium::glutin::window::WindowBuilder::new()
        .with_title(settings::TITLE)
        .with_inner_size(glium::glutin::dpi::PhysicalSize::new(
            settings::WIDTH,
            settings::HEIGHT,
        ));

    let display = glium::Display::new(window, context, &events).unwrap();

    let mut objects = generate(&display);

    events.run(move |event, _, flow| {
        let instant = std::time::Instant::now();
        let delay = std::time::Duration::from_millis(5);

        *flow = glium::glutin::event_loop::ControlFlow::WaitUntil(instant + delay);

        match event {
            glium::glutin::event::Event::WindowEvent { event, .. } => match event {
                glium::glutin::event::WindowEvent::CloseRequested => {
                    *flow = glium::glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glium::glutin::event::Event::NewEvents(cause) => match cause {
                glium::glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glium::glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        update(&mut objects);
        collisions(&mut objects);
        adjust(&mut objects);

        let parameters = draw(&display);

        let mut frame = display.draw();

        frame.clear_color(0.96, 0.96, 0.96, 1.0);

        for index in 0..objects.len() {
            objects[index].render(&mut frame, &parameters);
        }

        frame.finish().unwrap();
    });
}
