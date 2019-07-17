use std::os::raw::c_char;
use lbfgsb_sys::string as ffi;

#[inline]
pub fn stringfy(task: &mut [c_char]) {
    unsafe {
        ffi::stringfy_(task.as_mut_ptr());
    }
}
