extern crate interp_util;
extern crate rand;
extern crate gnuplot;
extern crate statistics;

use gnuplot::*;
use interp_util::*;
use rand::distributions::{IndependentSample, Normal};
use std::cmp::min;
use std::cmp::max;
use statistics::*;
use rand::Rng;

fn generate_function(pts: &Vec<f64>, a: f64, b: f64, err: f64) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, err);
    pts.iter().map(|x| (a*x + b + normal.ind_sample(&mut rng))).collect()
}

fn calc_derivative(xs: &[f64], ys: &[f64]) -> Vec<f64> {
    let mut res = Vec::new();
    for i in 1..xs.len() - 1 {
        let t = xs[i+1]- xs[i-1];
        res.push((ys[i-1] + ys[i+1] - 2.0 * ys[i]) / (t*t));
    }
    res
}

fn plot_line_data(a: f64, b: f64, x: &[f64], y: &[f64]) {
    let first_pt = x[0];
    let last_pt = x[x.len() - 1];
    let mut fg = Figure::new();
    fg.axes2d()
        .set_size(1.0, 1.0)
        .set_x_ticks(Some((Auto, 1)), &[Mirror(false)], &[])
        .set_y_ticks(Some((Auto, 1)), &[Mirror(false)], &[])
        .points(x, y, &[Caption("Function with erros"), PointSize(1.0), Color("red")])
        .lines(&[first_pt, last_pt], &[a * first_pt + b, a * last_pt + b],
            &[Caption("Reference function"), LineWidth(1.5), Color("green")]);

    fg.set_terminal("pngcairo", "line_data.png");
    fg.show();
}

fn main() {
    let a = 1.5;
    let b = 1.0;
    let err_sigma = 1.0;
    let outliers = 3;
    let xs = linspace(0.0, 100.0, 100);
    let mut ys = generate_function(&xs, a, b, err_sigma);
    for i in 0..outliers {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range::<usize>(0, ys.len());
        let err = rng.gen_range(err_sigma * 10.0, err_sigma * 20.0) * (2 * rng.gen_range::<i32>(0, 2) - 1) as f64;
        println!("Outlier #{}: {}, error: {}", i + 1, idx, err);
        ys[idx] += err;
    }
    let der = calc_derivative(&xs, &ys);

    plot_line_data(a, b, &xs, &ys);

    let der_mean = mean(&der);
    let der_variance = variance(&der);
    println!("Derivative mean: {}\nDerivative variance: {}", der_mean, der_variance);

    for (i, d) in der.iter().enumerate() {
        if (d - der_mean).abs() > 3.0 * der_variance {
            println!("Possible outlier: {}", i + 1);
        }
    }

    let max_der = der.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
    let min_der = der.iter().max_by(|x, y| y.partial_cmp(x).unwrap()).unwrap();
    let box_num = 50;
    let step = (max_der - min_der) / box_num as f64;
    let mut boxes = (0..box_num).map(|_| 0.0).collect::<Vec<_>>();
    let box_centers = (0..box_num).map(|i| min_der + step / 2.0 + step * i as f64).collect::<Vec<_>>();
    for d in der.iter() {
        let t = (d - min_der) / (max_der - min_der);
        let mut idx = (t * box_num as f64) as i32;
        idx = max(min(box_num - 1, idx), 0);
        boxes[idx as usize] += 1.0;
    }
    
    let mut fg = Figure::new();
    fg.axes2d()
        .set_size(1.0, 1.0)
        .set_x_ticks(Some((Auto, 1)), &[Mirror(false)], &[])
        .set_y_ticks(Some((Auto, 1)), &[Mirror(false)], &[])
        .boxes(&box_centers, &boxes, &[]);

    fg.set_terminal("pngcairo", "der_distribution.png");
    fg.show();
}
