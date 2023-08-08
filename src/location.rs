use crate::object;
use object::{Object, ToObject};
use osmojit_sys;
use std::fmt;

/// A Location represents a location used when debugging jitted code.
#[derive(Copy, Clone)]
pub struct Location {
    ptr: *mut osmojit_sys::gcc_jit_location,
}

impl ToObject for Location {
    fn to_object(&self) -> Object {
        unsafe { object::from_ptr(osmojit_sys::gcc_jit_location_as_object(self.ptr)) }
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let obj = self.to_object();
        obj.fmt(fmt)
    }
}

/// # Safety
/// This function is unsafe because it dereferences a raw pointer.
pub unsafe fn from_ptr(ptr: *mut osmojit_sys::gcc_jit_location) -> Location {
    Location { ptr }
}

/// # Safety
/// This function is unsafe because it dereferences a raw pointer.
pub unsafe fn get_ptr(loc: &Location) -> *mut osmojit_sys::gcc_jit_location {
    loc.ptr
}
