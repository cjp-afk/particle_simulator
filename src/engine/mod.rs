use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
    dpi::{PhysicalSize, LogicalSize}
};
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use crate::engine::simulator::Simulator;

mod simulator;
pub mod renderer;
mod particle;

fn create_window(width: i32, height: i32) -> (EventLoop<()>, Window, PhysicalSize<u32>){
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Particle simulator")
        .with_inner_size(LogicalSize::new(width, height))
        .build(&event_loop)
        .unwrap();

    let size = window.inner_size();

    (event_loop, window, size)
}

pub async fn run() {
    let (event_loop, window, size) = create_window(1000, 1000);

    let simulator = Simulator::new(&window, 100).await;

    let mut frame_count = 0;
    let mut last_update_time = std::time::Instant::now();
    let mut delta_time = std::time::Instant::now();

    event_loop.run(|event, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                        ..
                    } => control_flow.exit(),

                    WindowEvent::RedrawRequested => {
                        if last_update_time.elapsed().as_secs_f32() >= 1.0 {
                            let title = format!("FPS: {}", frame_count);
                            window.set_title(&title);
                            frame_count = 0;
                            last_update_time = std::time::Instant::now();
                        }

                        window.request_redraw();
                        if delta_time.elapsed().as_secs_f32() >= 0.001 {
                            simulator.renderer.update();

                            match simulator.renderer.do_pass() {
                                Ok(_) => {}
                                _ => {}
                            }
                            frame_count += 1;
                            delta_time = std::time::Instant::now();
                        }

                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }).expect("Panic");
}