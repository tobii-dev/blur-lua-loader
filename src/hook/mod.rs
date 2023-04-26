use std::ffi::{c_char, c_int, c_void};

use windows::Win32::System::{
	LibraryLoader::GetModuleHandleA,
	Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS},
};

use mlua_sys::lua_State;

///Addy of the call instruction
const _ADDY_CALL: usize = 0x966DB9;

unsafe extern "C" fn print_api(s: *mut lua_State) -> c_int {
	let argc = mlua_sys::lua_gettop(s);
	let mut output = "".to_string();
	for idx in 1..=argc {
		if !output.is_empty() {
			output.push_str(", ");
		}
		let v = match mlua_sys::lua_type(s, idx) {
			mlua_sys::LUA_TNONE => "(LUA_TNONE)".to_string(),
			mlua_sys::LUA_TNIL => "(LUA_TNIL)".to_string(),
			mlua_sys::LUA_TBOOLEAN => {
				if mlua_sys::lua_toboolean(s, idx) != 0 {
					"(LUA_TBOOLEAN) = true".to_string()
				} else {
					"(LUA_TBOOLEAN) = false".to_string()
				}
			}
			mlua_sys::LUA_TLIGHTUSERDATA => "(LUA_TLIGHTUSERDATA)".to_string(),
			mlua_sys::LUA_TNUMBER => {
				let n = mlua_sys::lua_tonumber(s, idx);
				std::format!("(LUA_TNUMBER) = {n}")
			}
			mlua_sys::LUA_TSTRING => {
				let t = mlua_sys::lua_tostring(s, idx);
				let t = std::ffi::CStr::from_ptr(t).to_str().unwrap();
				std::format!("(LUA_TSTRING) = {t}")
			}
			mlua_sys::LUA_TTABLE => {
				let t = mlua_sys::lua_gettable(s, idx);
				std::format!("(LUA_TTABLE) = {t}")
			}
			mlua_sys::LUA_TFUNCTION => "(LUA_TFUNCTION)".to_string(),
			mlua_sys::LUA_TUSERDATA => "(LUA_TUSERDATA)".to_string(),
			mlua_sys::LUA_TTHREAD => "(LUA_TTHREAD)".to_string(),
			_ => {
				let t = mlua_sys::luaL_typename(s, idx);
				let t = std::ffi::CStr::from_ptr(t).to_str().unwrap();
				std::format!("({t})")
			}
		};
		output.push_str(&v);
	}
	log::debug!("[API] {output}");
	0
}

unsafe extern "C" fn loadbuffer(
	s: *mut lua_State,
	buff: *const c_char,
	sz: usize,
	name: *const c_char,
) -> c_int {
	static mut ORG: Option<*mut lua_State> = None;
	if ORG.is_none() {
		ORG = Some(s);
	};

	mlua_sys::lua_pushcfunction(s, print_api);
	let v = std::ffi::CString::new("print").unwrap();
	mlua_sys::lua_setglobal(s, v.as_ptr());

	let r = mlua_sys::luaL_loadbuffer(s, buff, sz, name);

	let org_state = ORG.unwrap();
	if org_state != s {
		log::trace!("loadbuffer(): called on new lua_State {s:#?}");
	} else {
		let sz_buff = std::ffi::CStr::from_ptr(buff).to_str().unwrap();
		let sz_name = std::ffi::CStr::from_ptr(name).to_str().unwrap();
		if sz_buff.eq(sz_name) {
			log::trace!("loadbuffer(): called for \"{sz_buff}\"");
		} else {
			log::trace!("loadbuffer(): called for \"{sz_buff}\" -> \"{sz_name}\"");
		}
	}
	r
}

pub fn hook() {
	let ptr_base: *mut c_void = unsafe { GetModuleHandleA(windows::core::PCSTR::null()) }
		.unwrap()
		.0 as *mut c_void;
	dbg!(ptr_base);

	let ptr_dst: *mut c_void = loadbuffer as *mut c_void;

	let ptr_src = ptr_base.wrapping_offset(_ADDY_CALL.try_into().unwrap());
	const INS_CALL_LEN: isize = 5;
	let ptr_src_next = ptr_src.wrapping_offset(INS_CALL_LEN);

	dbg!(ptr_base);
	dbg!(ptr_src_next);

	let rel_jmp: isize = ptr_dst as isize - ptr_src_next as isize;
	let flags: *mut PAGE_PROTECTION_FLAGS = &mut PAGE_PROTECTION_FLAGS::default();
	let _r = unsafe {
		VirtualProtect(
			ptr_src,
			INS_CALL_LEN.try_into().unwrap(),
			PAGE_EXECUTE_READWRITE,
			flags,
		)
	};
	let ov: *mut isize = ptr_src.wrapping_add(1) as _;
	unsafe { ov.write(rel_jmp) };
	let _r = unsafe { VirtualProtect(ptr_src, INS_CALL_LEN.try_into().unwrap(), *flags, flags) };
}
