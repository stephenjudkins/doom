use oxidoom::*;

use std::rc::Rc;

use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct WinitDoom {
    window: Option<Rc<Window>>,
    surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
}

impl winit::application::ApplicationHandler for WinitDoom {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Rc::new(
            event_loop
                .create_window(
                    winit::window::Window::default_attributes()
                        .with_inner_size(winit::dpi::Size::Physical(PhysicalSize {
                            width: WIDTH,
                            height: HEIGHT,
                        }))
                        .with_resizable(false),
                )
                .unwrap(),
        );

        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

        self.window = Some(window);
        self.surface = Some(surface)
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            winit::event::WindowEvent::RedrawRequested => {
                let window = self.window.as_ref().unwrap();
                tick();
                let mut doom = GLOBAL_DOOM.lock().unwrap();
                if doom.redraw_requested {
                    let surface = self.surface.as_mut().unwrap();
                    let mut buffer = surface.buffer_mut().unwrap();

                    for i in 0..PIXELS {
                        buffer[i] = doom.display_buffer[i]
                    }

                    buffer.present().unwrap();
                    doom.redraw_requested = false;
                }

                window.request_redraw();
            }
            winit::event::WindowEvent::KeyboardInput { event: _event, .. } => {
                // eprintln!("{event:?}");
            }
            _ => (),
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        _event: winit::event::DeviceEvent,
    ) {
        // eprintln!("{event:?}");
    }
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    init_global_doom();

    let mut app = WinitDoom {
        window: None,
        surface: None,
    };
    event_loop.run_app(&mut app).unwrap();
}
