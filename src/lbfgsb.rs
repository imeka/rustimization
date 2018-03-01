use lbfgsb_sys::lbfgsb as ffi;

#[inline]
pub fn step(
    n: i32,
    m: i32,
    x: &mut [f64],
    l: &[f64],
    u: &[f64],
    nbd: &[i32],
    f: f64,
    g: &[f64],
    factr: f64,
    pgtol: f64,
    wa: &mut [f64],
    iwa: &mut [i32],
    task: &mut [i8],
    iprint: i32,
    csave: &mut [i8],
    lsave: &mut [i32],
    isave: &mut [i32],
    dsave: &mut [f64])
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
