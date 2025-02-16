use cstr::cstr;
use mlua_sys::{
	luaL_dofile, luaL_loadbuffer, lua_State, lua_pushcfunction, lua_setglobal, luaopen_package,
	luaopen_string,
};
use std::ffi::{c_char, c_int, c_void};

use windows::Win32::System::Memory::{
	VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
};

use crate::hook::api;

unsafe extern "C" fn loadbuffer(
	s: *mut lua_State,
	buff: *const c_char,
	sz: usize,
	name: *const c_char,
) -> c_int {
	#![allow(static_mut_refs)]
	static mut FIRST_LUA_STATE: Option<*mut lua_State> = None;

	// native lua print by the Blur.exe
	lua_pushcfunction(s, api::print_debug);
	lua_setglobal(s, cstr!("print").as_ptr());

	// "our" print used by plugins
	lua_pushcfunction(s, api::print_api);
	lua_setglobal(s, cstr!("print_api").as_ptr());

	// set bit to allow solo racing
	lua_pushcfunction(s, api::solo);
	lua_setglobal(s, cstr!("solo").as_ptr());

	// used for coding horrors
	lua_pushcfunction(s, api::notify);
	lua_setglobal(s, cstr!("notify").as_ptr());

	// used for fps limiter
	lua_pushcfunction(s, api::set_fps);
	lua_setglobal(s, cstr!("set_fps").as_ptr());

	if FIRST_LUA_STATE.is_none() {
		log::trace!("Hooked luaL_loadbuffer(lua_State = {s:#?})");
		luaopen_package(s);
		luaopen_string(s);
		luaL_dofile(s, cstr!("amax/init.luac").as_ptr()); // compiled, used for testing
		luaL_dofile(s, cstr!("amax/init.lua").as_ptr()); //TODO: compile me
		FIRST_LUA_STATE = Some(s);
	};

	luaL_loadbuffer(s, buff, sz, name)
}

/// Hooks Blur.exe original luaL_loadbuffer() function with our custom one.
/// It accomplishes this by overwriting a specific CALL instruction in the Blur.exe process.
pub fn set_hook_loadbuffer(ptr_module_base: *mut c_void) {
	let ptr_dst: *const c_void = loadbuffer as _;

	/// Address of the call instruction to original luaL_loadbuffer
	const ADDY_CALL: isize = 0x966DB9;
	/// number of bytes this rel. call instruction takes (E8 ?? ?? ?? ??)
	const INS_CALL_LEN: isize = 5;

	let ptr_src = ptr_module_base.wrapping_offset(ADDY_CALL);
	// first byte is the opcode, we are overwriting the jmp target bytes
	let ptr: *mut isize = ptr_src.wrapping_add(1) as _;

	// because the jump is relative to the NEXT instruction
	let ptr_src_next = ptr_src.wrapping_offset(INS_CALL_LEN);
	let rel_jmp: isize = ptr_dst as isize - ptr_src_next as isize;

	// Windows will be angry if we write to protected memory!
	let src_flags = &mut PAGE_PROTECTION_FLAGS::default();
	let tmp_flags = PAGE_EXECUTE_READWRITE;
	unsafe { VirtualProtect(ptr_src, INS_CALL_LEN as usize, tmp_flags, src_flags).unwrap() };
	// finally overwriting the jump
	unsafe { ptr.write(rel_jmp) };
	// restore original PAGE_PROTECTION_FLAGS
	unsafe { VirtualProtect(ptr_src, INS_CALL_LEN as usize, *src_flags, src_flags).unwrap() };
}
