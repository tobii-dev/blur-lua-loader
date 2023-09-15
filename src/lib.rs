#![cfg(windows)]

pub mod hook;

use std::ffi::c_void;

use crate::hook::loadbuffer::set_hook_loadbuffer;
use windows::{core::PCSTR, Win32::System::LibraryLoader::GetModuleHandleA};

use simplelog::*;

use blur_plugins_core::{BlurAPI, BlurEvent, BlurPlugin};

pub static mut API: Option<Box<&mut dyn BlurAPI>> = None;

#[repr(C)]
pub struct MyLuaHooksPlugin {}

impl BlurPlugin for MyLuaHooksPlugin {
	fn name(&self) -> &'static str {
		"MyLuaHooksPlugin!"
	}

	fn on_event(&self, _event: &BlurEvent) {
		//log::info!("{}: {:?}", &self.name(), &_event);
	}

	fn free(&self) {
		// idk should we do something here?
	}
}

#[no_mangle]
fn plugin_init(api: &'static mut dyn BlurAPI) -> Box<dyn BlurPlugin> {
	unsafe { API = Some(Box::new(api)) }
	let cfg = ConfigBuilder::new()
		.set_time_offset_to_local()
		.unwrap()
		.build();

	CombinedLogger::init(vec![
		TermLogger::new(
			LevelFilter::Trace,
			cfg,
			TerminalMode::Mixed,
			ColorChoice::Auto,
		),
		WriteLogger::new(
			LevelFilter::Trace,
			Config::default(),
			std::fs::File::create(".\\amax\\log\\lua_hooks.log")
				.expect("Couldn't create log file: .\\amax\\log\\lua_hooks.log"),
		),
	])
	.unwrap();
	log_panics::init();

	let ptr_base: *mut c_void = unsafe { GetModuleHandleA(PCSTR::null()) }.unwrap().0 as _;
	set_hook_loadbuffer(ptr_base);

	Box::new(MyLuaHooksPlugin {})
}
