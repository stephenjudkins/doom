use std::{
    os::raw,
    ptr::null,
    rc::Rc,
    sync::Mutex,
    thread::sleep,
    time::{Duration, Instant},
};

use once_cell::sync::Lazy;
use softbuffer::Surface;
use winit::dpi::PhysicalSize;
use winit::window::Window;

static START_TIME: Lazy<Instant> = Lazy::new(|| Instant::now());

#[no_mangle]
extern "C" fn DG_Init() {}

const WIDTH: u32 = 640;
const HEIGHT: u32 = 400;

static REDRAW_REQUESTED: Mutex<bool> = Mutex::new(false);

#[no_mangle]
extern "C" fn DG_DrawFrame() {
    let mut redraw_requested = REDRAW_REQUESTED.lock().unwrap();
    *redraw_requested = true;
}

#[no_mangle]
extern "C" fn DG_SleepMs(ms: u32) {
    sleep(Duration::from_millis(ms as u64));
}

#[no_mangle]
extern "C" fn DG_GetTicksMs() -> u32 {
    u32::try_from(START_TIME.elapsed().as_millis())
        .expect("Can't fit passed milliseconds into u32!")
}

#[no_mangle]
extern "C" fn DG_GetKey(_pressed: *mut raw::c_int, _key: *mut raw::c_uchar) -> raw::c_int {
    0
}

#[no_mangle]
extern "C" fn DG_SetWindowTitle(_title: *const u8) {}

#[no_mangle]
static mut DG_ScreenBuffer: *const u32 = std::ptr::null();

extern "C" {
    fn doomgeneric_Create(argc: usize, argv: *const *const u8);
    fn doomgeneric_Tick();
}

#[derive(Default)]
struct Doom {
    window: Option<Rc<Window>>,
    surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
}

impl winit::application::ApplicationHandler for Doom {
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

                unsafe {
                    doomgeneric_Tick();
                }

                let mut redraw_requested = REDRAW_REQUESTED.lock().unwrap();

                if *redraw_requested {
                    let surface: &mut Surface<Rc<Window>, Rc<Window>> =
                        self.surface.as_mut().unwrap();
                    let mut buffer = surface.buffer_mut().unwrap();

                    unsafe {
                        std::ptr::copy(
                            DG_ScreenBuffer,
                            buffer.as_mut_ptr(),
                            (WIDTH * HEIGHT) as usize,
                        );
                    }

                    buffer.present().unwrap();

                    *redraw_requested = false;
                }

                window.request_redraw();
            }
            _ => (),
        }
    }

    fn new_events(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        let _ = (event_loop, cause);
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: ()) {
        let _ = (event_loop, event);
    }

    fn device_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        let _ = (event_loop, device_id, event);
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let _ = event_loop;
    }

    fn suspended(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let _ = event_loop;
    }

    fn exiting(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let _ = event_loop;
    }

    fn memory_warning(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let _ = event_loop;
    }
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    unsafe {
        doomgeneric_Create(0, null());
    }

    let mut app = Doom::default();
    event_loop.run_app(&mut app).unwrap();
}
