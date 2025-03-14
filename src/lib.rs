use std::{
    os::raw,
    sync::Mutex,
    thread::sleep,
    time::{Duration, Instant},
};

use lazy_static::lazy_static;

pub struct Doom {
    pub start_time: Instant,
    pub redraw_requested: bool,
    pub display_buffer: [u32; PIXELS],
}

pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 400;
pub const PIXELS: usize = (WIDTH * HEIGHT) as usize;

pub fn init_global_doom() {
    unsafe {
        doomgeneric_Create(0, std::ptr::null());
    }
}

pub fn tick() {
    unsafe {
        doomgeneric_Tick();
    }
}

lazy_static! {
    pub static ref GLOBAL_DOOM: Mutex<Doom> = Mutex::new(Doom {
        start_time: Instant::now(),
        redraw_requested: false,
        display_buffer: [0; PIXELS],
    });
}

#[no_mangle]
extern "C" fn DG_Init() {}

#[no_mangle]
extern "C" fn DG_DrawFrame() {
    let mut doom = GLOBAL_DOOM.lock().unwrap();
    doom.redraw_requested = true;

    unsafe {
        std::ptr::copy(
            DG_ScreenBuffer,
            doom.display_buffer.as_mut_ptr(),
            (WIDTH * HEIGHT) as usize,
        );
    }
}

#[no_mangle]
extern "C" fn DG_SleepMs(ms: u32) {
    sleep(Duration::from_millis(ms as u64));
}

#[no_mangle]
extern "C" fn DG_GetTicksMs() -> u32 {
    let doom = GLOBAL_DOOM.lock().unwrap();

    u32::try_from(doom.start_time.elapsed().as_millis())
        .expect("Can't fit passed milliseconds into u32!")
}

#[no_mangle]
extern "C" fn DG_GetKey(_pressed: *mut raw::c_int, _key: *mut raw::c_uchar) -> raw::c_int {
    0
}

#[no_mangle]
extern "C" fn DG_SetWindowTitle(_title: *const u8) {}

#[no_mangle]
pub static mut DG_ScreenBuffer: *const u32 = std::ptr::null();

extern "C" {
    pub fn doomgeneric_Create(argc: usize, argv: *const *const u8);
    pub fn doomgeneric_Tick();
}
