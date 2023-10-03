#![cfg(windows)]

use blur_plugins_core::{BlurAPI, BlurEvent, BlurPlugin};
use simplelog::*;
use std::ffi::c_void;
use windows::{core::PCSTR, Win32::System::LibraryLoader::GetModuleHandleA};

pub mod hook;

pub static mut API: Option<Box<&mut dyn BlurAPI>> = None;

#[repr(C)]
pub struct MyLuaHooksPlugin {}

impl BlurPlugin for MyLuaHooksPlugin {
	fn name(&self) -> &'static str {
		"LuaHooksPlugin"
	}

	fn on_event(&self, _event: &BlurEvent) {}

	fn free(&self) {
		#[cfg(feature = "minhook")]
		{
			let r = unsafe { minhook_sys::MH_Uninitialize() };
			if r != minhook_sys::MH_OK {
				log::error!("minhook_sys::MH_Uninitialize() returns {r}");
			}
		}
		unsafe { crate::hook::loadbuffer::free_plugins() };
	}
}

#[no_mangle]
fn plugin_init(api: &'static mut dyn BlurAPI) -> Box<dyn BlurPlugin> {
	//SAFETY: Nah
	unsafe { API = Some(Box::new(api)) }

	let plugin = MyLuaHooksPlugin {};

	let cfg = ConfigBuilder::new()
		.set_time_offset_to_local()
		.unwrap()
		.build();
	let log_file = blur_plugins_core::create_log_file("lua_hooks.log").unwrap();
	CombinedLogger::init(vec![
		TermLogger::new(
			LevelFilter::Trace,
			cfg,
			TerminalMode::Mixed,
			ColorChoice::Auto,
		),
		WriteLogger::new(LevelFilter::Trace, Config::default(), log_file),
	])
	.unwrap();
	log_panics::init();

	let ptr_base: *mut c_void = unsafe { GetModuleHandleA(PCSTR::null()) }.unwrap().0 as _;

	if cfg!(feature = "minhook") {
		#[cfg(feature = "minhook")]
		{
			let r = unsafe { minhook_sys::MH_Initialize() };
			if r != minhook_sys::MH_OK {
				log::error!("minhook_sys::MH_Initialize() returns {r}");
			}
			hook::loadbuffer::set_min_hook_loadbuffer(ptr_base);
		}
	} else {
		hook::loadbuffer::set_hook_loadbuffer(ptr_base);
	}

	Box::new(plugin)
}
