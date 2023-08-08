use crate::lvalue;
use crate::lvalue::{LValue, ToLValue};
use crate::object;
use crate::object::{Object, ToObject};
use crate::rvalue;
use crate::rvalue::{RValue, ToRValue};
use osmojit_sys;
use std::fmt;

/// Parameter represents a parameter to a function. A series of parameteres
/// can be combined to form a function signature.
#[derive(Copy, Clone)]
pub struct Parameter {
    ptr: *mut osmojit_sys::gcc_jit_param,
}

impl ToObject for Parameter {
    fn to_object(&self) -> Object {
        unsafe { object::from_ptr(osmojit_sys::gcc_jit_param_as_object(self.ptr)) }
    }
}

impl fmt::Debug for Parameter {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let obj = self.to_object();
        obj.fmt(fmt)
    }
}

impl ToRValue for Parameter {
    fn to_rvalue(&self) -> RValue {
        unsafe {
            let ptr = osmojit_sys::gcc_jit_param_as_rvalue(self.ptr);
            rvalue::from_ptr(ptr)
        }
    }
}

impl ToLValue for Parameter {
    fn to_lvalue(&self) -> LValue {
        unsafe {
            let ptr = osmojit_sys::gcc_jit_param_as_lvalue(self.ptr);
            lvalue::from_ptr(ptr)
        }
    }
}

/// # Safety
/// This function is unsafe because it is possible to create an invalid
pub unsafe fn from_ptr(ptr: *mut osmojit_sys::gcc_jit_param) -> Parameter {
    Parameter { ptr }
}

/// # Safety
/// This function is unsafe because it is possible to create an invalid
pub unsafe fn get_ptr(loc: &Parameter) -> *mut osmojit_sys::gcc_jit_param {
    loc.ptr
}
