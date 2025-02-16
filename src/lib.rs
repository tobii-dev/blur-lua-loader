#![cfg(windows)]

pub mod hook;

use std::{
	ffi::c_void,
	sync::{Mutex, OnceLock},
};

use crate::hook::loadbuffer::set_hook_loadbuffer;

use blur_plugins_core::{BlurAPI, BlurEvent, BlurPlugin};
use log::LevelFilter;

static API: OnceLock<Mutex<&mut dyn BlurAPI>> = OnceLock::new();

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
	init_logs();
	let ptr_base: *mut c_void = api.get_exe_base_ptr();
	//SAFETY: Nah
	API.set(Mutex::new(api)).map_err(|_| ()).unwrap();

	let plugin = MyLuaHooksPlugin {};
	set_hook_loadbuffer(ptr_base);

	Box::new(plugin)
}

fn init_logs() {
	use simplelog::{
		ColorChoice, CombinedLogger, Config, ConfigBuilder, TermLogger, TerminalMode, WriteLogger,
	};
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
}
