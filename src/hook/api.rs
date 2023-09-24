use std::ffi::{c_int, c_uchar, c_void, CStr};

use blur_plugins_core::BlurNotification;

use mlua_sys::{
	luaL_typename, lua_State, lua_gettable, lua_gettop, lua_toboolean, lua_tonumber, lua_tostring,
	lua_type, LUA_TBOOLEAN, LUA_TFUNCTION, LUA_TLIGHTUSERDATA, LUA_TNIL, LUA_TNONE, LUA_TNUMBER,
	LUA_TSTRING, LUA_TTABLE, LUA_TTHREAD, LUA_TUSERDATA,
};
use windows::{
	core::PCSTR,
	Win32::System::{
		LibraryLoader::GetModuleHandleA,
		Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS},
	},
};

pub unsafe extern "C-unwind" fn set_fps(s: *mut lua_State) -> c_int {
	let argc = lua_gettop(s);
	for idx in 1..=argc {
		match lua_type(s, idx) {
			LUA_TNONE => {}
			LUA_TNIL => {}
			LUA_TBOOLEAN => {
				let limit = lua_toboolean(s, idx) != 0;
				if !limit {
					if let Some(blur_api) = &mut crate::API {
						blur_api.set_fps(0.0);
					}
				}
			}
			LUA_TNUMBER => {
				let mut fps: f64 = lua_tonumber(s, idx);
				if !fps.is_finite() {
					continue;
				}
				if fps < 0.0 {
					fps = 0.0;
				}
				if let Some(blur_api) = &mut crate::API {
					log::trace!("set_fps(fps = {fps})");
					blur_api.set_fps(fps);
				}
			}
			_ => {}
		};
	}
	0
}

//TODO: Should this be here? Or should this be part of blur_api?
fn set_solo_racer_bit(bit: bool) {
	// log::info!("{}", bit);
	let ptr_base: *mut c_void =
		unsafe { GetModuleHandleA(PCSTR::null()) }.unwrap().0 as *mut c_void;
	const ADDY_BYTE_SOLO_RACER: isize = 0xE25800;

	let ptr_dst = ptr_base.wrapping_offset(ADDY_BYTE_SOLO_RACER);

	let flags: *mut PAGE_PROTECTION_FLAGS = &mut PAGE_PROTECTION_FLAGS::default();
	let _r = unsafe { VirtualProtect(ptr_dst, 1, PAGE_EXECUTE_READWRITE, flags) };
	let ov: *mut c_uchar = ptr_dst as _;
	unsafe { ov.write(if bit { 1 } else { 0 }) };
	let _r = unsafe { VirtualProtect(ptr_dst, 1, *flags, flags) };
}

pub unsafe extern "C-unwind" fn solo(s: *mut lua_State) -> c_int {
	let argc = lua_gettop(s);
	for idx in 1..=argc {
		match lua_type(s, idx) {
			LUA_TNONE => {}
			LUA_TNIL => {}
			LUA_TBOOLEAN => {
				let bit = lua_toboolean(s, idx) != 0;
				set_solo_racer_bit(bit);
			}
			_ => {}
		};
	}
	0
}

pub unsafe extern "C-unwind" fn print_api(s: *mut lua_State) -> c_int {
	let argc = lua_gettop(s);
	let mut output = "".to_string();
	for idx in 1..=argc {
		if !output.is_empty() {
			output.push_str(", ");
		}
		let v = match lua_type(s, idx) {
			LUA_TNONE => "None".to_string(),
			LUA_TNIL => "nil".to_string(),
			LUA_TBOOLEAN => {
				if lua_toboolean(s, idx) != 0 {
					"true".to_string()
				} else {
					"false".to_string()
				}
			}
			LUA_TLIGHTUSERDATA => "LIGHTUSERDATA".to_string(),
			LUA_TNUMBER => {
				let n = lua_tonumber(s, idx);
				std::format!("{n}")
			}
			LUA_TSTRING => {
				let t = lua_tostring(s, idx);
				let t = CStr::from_ptr(t).to_str().unwrap();
				std::format!("{t}")
			}
			LUA_TTABLE => {
				let t = lua_gettable(s, idx);
				std::format!("table (@{t})")
			}
			LUA_TFUNCTION => "function".to_string(),
			LUA_TUSERDATA => "USERDATA".to_string(),
			LUA_TTHREAD => "THREAD".to_string(),
			_ => {
				let t = luaL_typename(s, idx);
				let t = CStr::from_ptr(t).to_str().unwrap();
				std::format!("({t})")
			}
		};
		output.push_str(&v);
	}

	log::debug!("[Lua] {output}");

	{
		use colored::*;
		let output = output.cyan();
		std::println!("{output}");
	}
	0
}

pub unsafe extern "C-unwind" fn print_debug(s: *mut lua_State) -> c_int {
	let argc = lua_gettop(s);
	let mut output = "".to_string();
	for idx in 1..=argc {
		if !output.is_empty() {
			output.push_str(", ");
		}
		let v = match lua_type(s, idx) {
			LUA_TNONE => "(LUA_TNONE)".to_string(),
			LUA_TNIL => "(LUA_TNIL)".to_string(),
			LUA_TBOOLEAN => {
				if lua_toboolean(s, idx) != 0 {
					"(LUA_TBOOLEAN) = true".to_string()
				} else {
					"(LUA_TBOOLEAN) = false".to_string()
				}
			}
			LUA_TLIGHTUSERDATA => "(LUA_TLIGHTUSERDATA)".to_string(),
			LUA_TNUMBER => {
				let n = lua_tonumber(s, idx);
				std::format!("(LUA_TNUMBER) = {n}")
			}
			LUA_TSTRING => {
				let t = lua_tostring(s, idx);
				let t = CStr::from_ptr(t).to_str().unwrap();
				std::format!("(LUA_TSTRING) = {t}")
			}
			LUA_TTABLE => {
				let t = lua_gettable(s, idx);
				std::format!("(LUA_TTABLE) = {t}")
			}
			LUA_TFUNCTION => "(LUA_TFUNCTION)".to_string(),
			LUA_TUSERDATA => "(LUA_TUSERDATA)".to_string(),
			LUA_TTHREAD => "(LUA_TTHREAD)".to_string(),
			_ => {
				let t = luaL_typename(s, idx);
				let t = CStr::from_ptr(t).to_str().unwrap();
				std::format!("({t})")
			}
		};
		output.push_str(&v);
	}

	//log::debug!("[Lua Debug] {output}");

	{
		use colored::*;
		let output = output.green();
		std::println!("{output}");
	}
	0
}

//TODO: structured events from Lua to blur_api. Considering an events Table
pub unsafe extern "C-unwind" fn notify(s: *mut lua_State) -> c_int {
	let argc = lua_gettop(s);
	for idx in 1..=argc {
		match lua_type(s, idx) {
			LUA_TNONE => {
				todo!()
			}
			LUA_TNIL => {
				todo!()
			}
			LUA_TBOOLEAN => {
				if lua_toboolean(s, idx) != 0 {
					todo!() // true
				} else {
					todo!() // false
				}
			}
			LUA_TLIGHTUSERDATA => todo!(),
			LUA_TNUMBER => {
				let n = lua_tonumber(s, idx);
				if !n.is_normal() {
					continue;
				}
				let n: i64 = n as i64;

				if let Some(blur_api) = &mut crate::API {
					// FIXME: this makes no sense right now.
					let notif = match n {
						0 => BlurNotification::Nothing,
						1 => BlurNotification::LoginStart,
						2 => BlurNotification::LoginEnd { success: true }, // FIXME: what success??
						3 => BlurNotification::Screen { name: "".to_string() }, // FIXME: what Screen name??
						_ => {
							log::warn!("Got unknown notify({n}) event from Lua?");
							return 0;
						}
					};
					blur_api.notify(notif);
				}
			}
			LUA_TSTRING => {
				let t = lua_tostring(s, idx);
				let _t = CStr::from_ptr(t).to_str().unwrap();
				todo!()
			}
			LUA_TTABLE => {
				let _t = lua_gettable(s, idx);
				todo!()
			}
			LUA_TFUNCTION => todo!(),
			LUA_TUSERDATA => todo!(),
			LUA_TTHREAD => todo!(),
			_ => {
				let t = luaL_typename(s, idx);
				let _t = CStr::from_ptr(t).to_str().unwrap();
				//todo!()
			}
		};
	}
	//log::debug!("notify");
	0
}
