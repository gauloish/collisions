extern crate glium;

fn main() {
    use glium::glutin;
    use glium::Surface;

    let mut events = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(625.0, 625.0))
        .with_title("Collisions");
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events).unwrap();

    events.run(move |ev, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let mut frame = display.draw();

        frame.clear_color(0.0, 0.0, 1.0, 1.0);

        frame.finish().unwrap();

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }
    });
}
