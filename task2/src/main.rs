extern crate nalgebra as na;

use na::core::{DMatrix, MatrixVec};
use std::f64;

fn polynomial_error(x0: f64, pts: &Vec<f64>) -> f64 {
    let mut res = 0.0;

    for i in 0..pts.len() {
        let mut tmp = 1.0;
        for j in 0..pts.len() {
            if j != i {
                tmp = tmp * (x0 - pts[j]) / (pts[i] - pts[j]);
            }
        }
        res += tmp.abs();
    }

    res
}

fn polynomial_derivative_error(x0: f64, pts: &Vec<f64>) -> f64 {
    let mut res = 0.0;

    for i in 0..pts.len() {
        let mut t1 = 0.0;
        let mut t2 = 0.0;
        assert!(false);

    }

    res
}

fn linspace_vec(min: f64, max: f64, num: usize) -> Vec<f64> {
    let dt = (max - min) / ((num - 1) as f64);
    let mut pts = Vec::with_capacity(num);
    for i in 0..num {
        pts.push(min + dt * i as f64);
    }

    pts
}

fn chebyshev_knots(k: usize) -> Vec<f64> {
    let mut res = Vec::new();

    for m in 0..k {
        res.push((f64::consts::PI * ((2 * m + 1) as f64) / ((2 * k) as f64)).cos());
    }

    res
}

fn linspace(min: f64, max: f64, num: usize) -> DMatrix<f64> {
    let dt = (max - min) / ((num - 1) as f64);
    let mut pts = Vec::with_capacity(num);
    for i in 0..num {
        pts.push(min + dt * i as f64);
    }

    DMatrix::from_data(MatrixVec::new(na::Dynamic::new(1), na::Dynamic::new(num), pts))
}


fn main() {
    let N = 11;
    let pts = linspace_vec(-1.0, 1.0, 301);
    let u_grid = linspace_vec(-1.0, 1.0, N); // uniform grid
    //let c_grid = Vec::new();                 // Chebyshev grid
    let mut base = Vec::with_capacity(pts.len());
    for x in pts {
        base.push(polynomial_error(x, &u_grid));
    }
    println!("{:?}", base);



    println!("Hello, world!");
    println!("{:?}", linspace(-1.0, 1.0, 4));
    println!("{:?}", chebyshev_knots(5));
}
