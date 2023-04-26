use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(
	dll_module: windows::Win32::Foundation::HMODULE,
	call_reason: u32,
	_reserved: *mut std::ffi::c_void,
) -> i32 {
	match call_reason {
		DLL_PROCESS_ATTACH => crate::init(dll_module),
		DLL_PROCESS_DETACH => crate::free(dll_module),
		_ => (),
	}
	true.into()
}
