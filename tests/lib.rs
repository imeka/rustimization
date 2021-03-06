extern crate rustimization;

use rustimization::lbfgsb_minimizer::Lbfgsb;

#[test]
fn test_lbfgsb() {
    let f = |x: &Vec<f64>| (x[0] + 4.0).powf(2.0);
    let g = |x: &Vec<f64>| vec![2.0 * (x[0] + 4.0)];
    let mut x = vec![40.0f64];

    {
        let mut fmin = Lbfgsb::new(&mut x, &f, Some(&g));
        //fmin.set_lower_bound(0, 10.0);
        fmin.set_upper_bound(0, 100.0);
        fmin.set_verbosity(-1);
        fmin.max_iteration(100);
        fmin.minimize();
    }

    assert_eq!(x[0], -4.0);
}

#[test]
fn test_init() {
    let mut x = vec![40.0f64];
    inner(&mut x);
}

fn inner(x: &mut Vec<f64>) {
    let f = |x: &Vec<f64>| (x[0] + 4.0).powf(2.0);
    let mut fmin = Lbfgsb::new(x, &f, None);
    fmin.minimize();
}
