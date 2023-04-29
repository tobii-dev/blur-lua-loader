#![cfg(windows)]

pub mod dll;
pub mod hook;

use std::ffi::c_void;

use crate::hook::loadbuffer::set_hook_loadbuffer;
use windows::{
	core::PCSTR,
	Win32::{Foundation::HMODULE, System::LibraryLoader::GetModuleHandleA},
};

use simplelog::*;

pub fn init(module: HMODULE) {
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
	log::info!("Hi from lua_hooks: {module:X?}");

	let ptr_base: *mut c_void = unsafe { GetModuleHandleA(PCSTR::null()) }.unwrap().0 as _;
	set_hook_loadbuffer(ptr_base);
}

pub fn free(module: HMODULE) {
	log::info!("Bye from lua_hooks: {module:X?}");
}
