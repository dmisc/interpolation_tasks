extern crate interp_util;
extern crate rand;
extern crate gnuplot;

use gnuplot::*;
use interp_util::*;
use rand::distributions::{IndependentSample, Normal};
use std::cmp::min;
use std::cmp::max;

fn generate_function(pts: &Vec<f64>, a: f64, b: f64, err_sigma: f64) -> Vec<(f64, f64)> {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, err_sigma);
    pts.iter().map(|x| (*x, a*x + b + normal.ind_sample(&mut rng))).collect()
}

fn calc_derivative(pts: &Vec<(f64, f64)>) -> Vec<f64> {
    let mut res = Vec::new();
    for i in 1..pts.len() - 1 {
        let t = pts[i+1].0 - pts[i-1].0;
        res.push((pts[i-1].1 + pts[i+1].1 - 2.0 * pts[i].1) / (t*t));
    }
    res
}

fn main() {
    println!("Hello, world!");
    let pts = linspace(0.0, 100.0, 100);
    let mut fpts = generate_function(&pts, 1.5, 1.0, 0.3);
    let l = fpts.len();
    fpts[l / 2].1 += 10.0;
    println!("{:?}", fpts);
    let der = calc_derivative(&fpts);
    println!("{:?}", der);

    let max_der = der.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
    let min_der = der.iter().max_by(|x, y| y.partial_cmp(x).unwrap()).unwrap();
    let box_num = 50;
    let step = (max_der - min_der) / box_num as f64;
    let mut boxes = (0..box_num).map(|_| 0.0).collect::<Vec<_>>();
    let box_centers = (0..box_num).map(|i| min_der + step / 2.0 + step * i as f64).collect::<Vec<_>>();
    println!("{:?}", boxes);
    for d in der.iter() {
        let t = (d - min_der) / (max_der - min_der);
        let mut idx = (t * box_num as f64) as i32;
        idx = max(min(box_num - 1, idx), 0);
        boxes[idx as usize] += 1.0;
    }
    for (i, b) in boxes.iter().enumerate() {
        println!("{}: {}", i, b);
    }
    println!("{:?}", box_centers);
    
    let mut fg = Figure::new();
    fg.axes2d()
        .set_size(1.0, 1.0)
        .set_x_ticks(Some((Auto, 1)), &[Mirror(false)], &[])
        .set_y_ticks(Some((Auto, 1)), &[Mirror(false)], &[])
        .boxes(&box_centers, &boxes, &[]);

    fg.set_terminal("pngcairo", "der_distribution.png");
    fg.show();
}
