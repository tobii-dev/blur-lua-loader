use std::sync::atomic::{AtomicU32, Ordering};

/// This gets set by the lua fps plugin
pub static TARGET_FPS: AtomicU32 = AtomicU32::new(0);

/// This gets called from the d3d9 Device fps limiter
#[no_mangle]
extern "C" fn get_fps() -> u32 {
	TARGET_FPS.load(Ordering::Relaxed)
}
