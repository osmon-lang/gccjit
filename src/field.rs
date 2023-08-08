use std::fmt;

use crate::object;
use crate::object::{Object, ToObject};

/// Field represents a field that composes structs or unions. A number of fields
/// can be combined to create either a struct or a union.
#[derive(Copy, Clone)]
pub struct Field {
    ptr: *mut osmojit_sys::gcc_jit_field,
}

impl ToObject for Field {
    fn to_object(&self) -> Object {
        unsafe { object::from_ptr(osmojit_sys::gcc_jit_field_as_object(self.ptr)) }
    }
}

impl fmt::Debug for Field {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let obj = self.to_object();
        obj.fmt(fmt)
    }
}

/// # Safety
/// This function is unsafe because it dereferences a raw pointer.
pub unsafe fn from_ptr(ptr: *mut osmojit_sys::gcc_jit_field) -> Field {
    Field { ptr }
}

/// # Safety
/// This function is unsafe because it dereferences a raw pointer.
pub unsafe fn get_ptr(f: &Field) -> *mut osmojit_sys::gcc_jit_field {
    f.ptr
}
