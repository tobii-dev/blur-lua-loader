#![feature(c_unwind)]
#![cfg(windows)]

pub mod dll;
pub mod hook;

use windows::Win32::Foundation::HMODULE;

use simplelog::*;

use crate::hook::hook;

pub fn init(module: HMODULE) {
	let cfg = ConfigBuilder::new()
		.set_time_offset_to_local()
		.unwrap()
		.build();

	let _logger = CombinedLogger::init(vec![
		TermLogger::new(
			LevelFilter::Trace,
			cfg,
			TerminalMode::Mixed,
			ColorChoice::Auto,
		),
		WriteLogger::new(
			LevelFilter::Trace,
			Config::default(),
			std::fs::File::create("lua_hooks.log").unwrap(),
		),
	])
	.unwrap();
	log_panics::init();
	log::info!("Hi from lua_hooks {module:?}");
	hook();
}

pub fn free(module: HMODULE) {
	log::info!("Bye from lua_hooks! {module:?}");
}
