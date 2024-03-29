use osmojit_sys;
use std::ffi::CStr;
use std::fmt;
use std::str;

/// Object represents the root of all objects in gccjit. It is not useful
/// in and of itself, but it provides the implementation for Debug
/// used by most objects in this library.
#[derive(Copy, Clone)]
pub struct Object {
    ptr: *mut osmojit_sys::gcc_jit_object,
}

impl fmt::Debug for Object {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        unsafe {
            let ptr = osmojit_sys::gcc_jit_object_get_debug_string(self.ptr);
            let cstr = CStr::from_ptr(ptr);
            let rust_str = str::from_utf8_unchecked(cstr.to_bytes());
            fmt.write_str(rust_str)
        }
    }
}

/// ToObject is a trait implemented by types that can be upcast to Object.
pub trait ToObject {
    fn to_object(&self) -> Object;
}

impl ToObject for Object {
    fn to_object(&self) -> Object {
        unsafe { from_ptr(self.ptr) }
    }
}

/// # Safety
/// This function is unsafe because it does not check that the pointer
pub unsafe fn from_ptr(ptr: *mut osmojit_sys::gcc_jit_object) -> Object {
    Object { ptr }
}

/// # Safety
/// This function is unsafe because it does not check that the pointer
pub unsafe fn get_ptr(object: &Object) -> *mut osmojit_sys::gcc_jit_object {
    object.ptr
}
