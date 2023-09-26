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
		"LuaHooksPlugin"
	}

	fn on_event(&self, _event: &BlurEvent) {}

	fn free(&self) {}
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
	set_hook_loadbuffer(ptr_base);

	Box::new(plugin)
}
