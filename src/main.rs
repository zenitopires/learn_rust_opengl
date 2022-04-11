extern crate glium;

use glium::{glutin, implement_vertex};

mod shader_utils;
use shader_utils::vertex::Vertex;
use shader_utils::read_vertex_shader_file::read_file;

fn main() {
    use glium::{glutin, Surface};

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let fragment_shader_src = read_file(
        "C:\\Users\\Zenito\\CLionProjects\\rust_opengl\\src\\content\\fragment_shader_src.glsl").unwrap();

    let vertex_shader_src = read_file(
        "C:\\Users\\Zenito\\CLionProjects\\rust_opengl\\src\\content\\vertex_shader_src.glsl").unwrap();

    let mut events_loop = glium::glutin::event_loop::EventLoop::new();

    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Mr. Slimpickings Jim");

    let cb = glium::glutin::ContextBuilder::new();

    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let program = glium::Program::from_source(
        &display, vertex_shader_src.as_str(), fragment_shader_src.as_str(), None).unwrap();

    events_loop.run(move |ev, _, control_flow| {
        let mut target = display.draw();
        target.clear_color(1.0, 0.5, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
        &Default::default()).unwrap();
        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    println!("Closing window");
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => ()
        }
    });
}
