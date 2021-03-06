use std::os::raw::{c_char, c_double, c_int};
use std::f64;
use std::ffi::CStr;
use lbfgsb::step;
use string::stringfy;

pub struct Lbfgsb<'a> {
    n: c_int,
    m: c_int,
    x: &'a mut Vec<c_double>,
    l: Vec<c_double>,
    u: Vec<c_double>,
    nbd: Vec<c_int>,
    f: &'a Fn(&Vec<c_double>) -> c_double,
    g: Option<&'a Fn(&Vec<c_double>) -> Vec<c_double>>,
    factr: c_double,
    pgtol: c_double,
    iwa: Vec<c_int>,
    task: Vec<c_char>,
    iprint: c_int,
    csave: Vec<c_char>,
    lsave: Vec<c_int>,
    isave: Vec<c_int>,
    dsave: Vec<c_double>,
    max_iter: u32
}

impl<'a> Lbfgsb<'a> {
    // Constructor requires three mendatory parameters which are the initial
    // solution, function and the gradient function
    pub fn new(
        x: &'a mut Vec<c_double>,
        f: &'a Fn(&Vec<c_double>) -> c_double,
        g: Option<&'a Fn(&Vec<c_double>) -> Vec<c_double>>
    ) -> Self {
        let n = x.len();
        Lbfgsb {
            n: n as i32,
            m: 5,
            x,
            l: vec![0.0; n],
            u: vec![0.0; n],
            nbd: vec![0; n],
            f,
            g,
            factr: 0.0,
            pgtol: 0.0,
            iwa: vec![0; 3 * n],
            task: vec![0; 60],
            iprint: -1,
            csave: vec![0; 60],
            lsave: vec![0, 0, 0, 0],
            isave: vec![0; 44],
            dsave: vec![0.0; 29],
            max_iter: 0,
        }
    }

    // This function starts the optimization algorithm
    pub fn minimize(&mut self) -> f64 {
        let n = self.x.len();
        let m = self.m as usize;
        let mut wa: Vec<c_double> = vec![0.0; 2 * m * n + 5 * n + 11 * m * m + 8 * m];

        let mut fval = 0.0;
        let mut gval = vec![0.0; n];
        let factr = self.factr / f64::EPSILON;

        // Converting fortran string "STRAT"
        stringfy(&mut self.task);

        loop {
            step(
                self.n, self.m, &mut self.x, &self.l, &self.u, &self.nbd, fval, &gval, factr,
                self.pgtol, &mut wa, &mut self.iwa, &mut self.task, self.iprint,
                &mut self.csave, &mut self.lsave, &mut self.isave, &mut self.dsave);

            // Converting to rust string
            let tsk = unsafe { CStr::from_ptr(self.task.as_ptr()).to_string_lossy() };
            if &tsk[0..2] == "FG" {
                fval = (self.f)(self.x);
                gval = match self.g {
                    Some(g) => g(self.x),
                    None => default_g(self.x, self.f, fval)
                };
            } else if &tsk[0..5] == "NEW_X"
                    && self.max_iter == 0
                    && self.dsave[11] <= 1.0e-10 * (1.0 + fval.abs()) {
                println!("THE PROJECTED GRADIENT IS SUFFICIENTLY SMALL");
                break;
            }
            if self.max_iter > 0 && self.isave[29] >= self.max_iter as i32 {
                break;
            }
            if &tsk[0..4] == "CONV" {
                // println!("convergence!");
                break;
            }
            if &tsk[0..5] == "ERROR" {
                println!("error in the input parameters");
            }
            if &tsk[0..8] == "ABNORMAL" {
                println!("ERROR: ABNORMAL TERMINATION");
                break;
            }
        }
        fval
    }

    // Returns the solution after minimization
    pub fn get_x(&self) -> Vec<c_double> {
        self.x.clone()
    }

    // Set lower bounds to a variable
    pub fn set_lower_bound(&mut self, index: usize, value: f64) {
        let var = &mut self.nbd[index];
        if *var == 1 || *var == 2 {
            println!("variable already has Lower Bound");
        } else {
            *var = (*var - 1).abs();
            self.l[index] = value;
        }
    }

    // Set upper bounds to a variable
    pub fn set_upper_bound(&mut self, index: usize, value: f64) {
        let var = &mut self.nbd[index];
        if *var == 2 || *var == 3 {
            println!("variable already has Lower Bound");
        } else {
            *var = 3 - *var;
            self.u[index] = value;
        }
    }

    // Set the verbosity level
    pub fn set_verbosity(&mut self, l: i32) {
        self.iprint = l;
    }

    // Set termination tolerance
    // 1.0e12 for low accuracy
    // 1.0e7  for moderate accuracy
    // 1.0e1  for extremely high accuracy
    pub fn set_termination_tolerance(&mut self, t: f64) {
        self.factr = t;
    }

    // Set tolerance of projection gradient
    pub fn set_tolerance(&mut self, t: f64) {
        self.pgtol = t;
    }

    // Set max iteration
    pub fn max_iteration(&mut self, i: u32) {
        self.max_iter = i;
    }

    // Set maximum number of variable metric corrections
    // The range 3 <= m <= 20 is recommended
    pub fn set_matric_correction(&mut self, m: i32) {
        self.m = m;
    }
}

fn default_g(x: &Vec<f64>, f: &Fn(&Vec<c_double>) -> c_double, f0: f64) -> Vec<f64> {
    let epsilon = 1e-08;
    let n = x.len();
    let mut grad = vec![0.0; n];
    let mut ei = vec![0.0; n];
    for i in 0..n {
        ei[i] = epsilon;
        for j in 0..n {
            ei[j] += x[j]
        }
        grad[i] = (f(&ei) - f0) / epsilon;
        for j in 0..n {
            ei[j] = 0.0;
        }
    }
    grad
}
