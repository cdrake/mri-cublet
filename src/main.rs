pub(crate) mod camera_controller;

use camera_controller::CameraController;
use glam::Vec3;
use wgpu_shape_renderer::render::{camera::OrbitCamera, state::State};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use futures::executor;
use loaders::json_loader::{load_json, log_init};

fn main() {
    //let before = std::time::Instant::now();
    log_init();
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Effcient wgpu cube")
        .build(&event_loop)
        .unwrap();

    //println!("Window shown in {:.2?}.", before.elapsed());
    // let json = executor::block_on(load_json("https://api.github.com/"));
    let size = window.inner_size();

    let mut camera = OrbitCamera::new(
        2.0,
        1.5,
        1.25,
        Vec3::new(0.0, 0.0, 0.0),
        size.width as f32 / size.height as f32,
    );
    camera.bounds.min_distance = Some(1.1);

    let mut camera_controller = CameraController::new(0.025, 0.6);

    // State::new uses async code, so we're going to wait for it to finish
    let mut state = pollster::block_on(State::new(&window, size.width, size.height, camera));

    //println!("Setup done in {:.2?}.", before.elapsed());

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    state.resize(physical_size.width, physical_size.height);
                    window.request_redraw();
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    state.resize(new_inner_size.width, new_inner_size.height);
                    window.request_redraw();
                }
                _ => {}
            },
            Event::DeviceEvent { ref event, .. } => {
                camera_controller.process_events(event, &window, &mut state.camera);
            }
            Event::RedrawRequested(_) => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => {
                        state.resize(state.width, state.height);
                        window.request_redraw();
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => {
                        eprintln!("{:?}", e);
                        // Request redraw to fix fix the issue.
                        window.request_redraw();
                    }
                }
            }
            _ => {}
        }
    });
}
