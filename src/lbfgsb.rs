use std::os::raw::{c_char, c_double, c_int};
use lbfgsb_sys::lbfgsb as ffi;

#[inline]
pub fn step(
    n: c_int,
    m: c_int,
    x: &mut [c_double],
    l: &[c_double],
    u: &[c_double],
    nbd: &[c_int],
    f: c_double,
    g: &[c_double],
    factr: c_double,
    pgtol: c_double,
    wa: &mut [c_double],
    iwa: &mut [c_int],
    task: &mut [c_char],
    iprint: c_int,
    csave: &mut [c_char],
    lsave: &mut [c_int],
    isave: &mut [c_int],
    dsave: &mut [c_double])
{
    unsafe {
        ffi::setulb_(
            &n,
            &m,
            x.as_mut_ptr(),
            l.as_ptr(),
            u.as_ptr(),
            nbd.as_ptr(),
            &f,
            g.as_ptr(),
            &factr,
            &pgtol,
            wa.as_mut_ptr(),
            iwa.as_mut_ptr(),
            task.as_mut_ptr(),
            &iprint,
            csave.as_mut_ptr(),
            lsave.as_mut_ptr(),
            isave.as_mut_ptr(),
            dsave.as_mut_ptr())
    }
}
