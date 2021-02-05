use wren_sys as ffi;

use std::mem::MaybeUninit;
use std::ffi::{CStr, CString};
use libc::{c_char, c_int};

extern "C" fn write_fn(vm: *mut ffi::WrenVM, text: *const c_char) {
	unsafe {
		print!("{}", CStr::from_ptr(text).to_str().unwrap());
	}
}

extern "C" fn error_fn(vm: *mut ffi::WrenVM, error_type: ffi::WrenErrorType, module: *const c_char, line: c_int, message: *const c_char) {
	unsafe {
		let module_str = CStr::from_ptr(module).to_str().unwrap();
		let message_str = CStr::from_ptr(message).to_str().unwrap();

		match error_type {
			ffi::WrenErrorType::Compile => println!("[{0} line {1}] [Error] {2}", module_str, line, message_str),
			ffi::WrenErrorType::Runtime => println!("[{0} line {1}] in {2}", module_str, line, message_str),
			ffi::WrenErrorType::StackTrace => println!("[Runtime Error] {}", module_str),
		}
	}
}

fn main() {
	unsafe {
		let mut config = MaybeUninit::<ffi::WrenConfiguration>::uninit();
		ffi::wrenInitConfiguration(config.as_mut_ptr());

		(*config.as_mut_ptr()).write_fn = write_fn;
		(*config.as_mut_ptr()).error_fn = error_fn;

		let vm = ffi::wrenNewVM(config.as_mut_ptr());

		let module = "my_module";
		let source = "System.print(\"I am running in a VM!\")";

		let module_cstr = CString::new(module).unwrap();
		let source_cstr = CString::new(source).unwrap();
		let result = ffi::wrenInterpret(vm, module_cstr.as_ptr(), source_cstr.as_ptr());

		match result {
			ffi::WrenInterpretResult::CompileError => println!("Compile Error!"),
			ffi::WrenInterpretResult::RuntimeError => println!("Runtime Error!"),
			ffi::WrenInterpretResult::Success => println!("Success"),
		}

		ffi::wrenFreeVM(vm);
	}
}

