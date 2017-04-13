extern crate gnuplot;

use std::f64;

use gnuplot::*;

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

fn linspace(min: f64, max: f64, num: usize) -> Vec<f64> {
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

fn plot(name: &str, x: &[f64], y: &[f64]) {
    let mut fg = Figure::new();

    fg.axes2d()
        .set_size(0.75, 1.0)
        .set_title(name, &[])
        .set_x_ticks(Some((Fix(1.0), 1)), &[Mirror(false)], &[])
        .set_y_ticks(Some((Fix(1.0), 1)), &[Mirror(false)], &[])
        .set_legend(Graph(1.0), Graph(0.5), &[Placement(AlignLeft, AlignCenter)], &[TextAlign(AlignRight)])
        .set_border(true, &[Left, Bottom], &[LineWidth(2.0)])
        .set_x_label("Abscissa", &[])
        .set_y_label("Ordinate", &[])
        .lines(x, y, &[Caption("base"), LineWidth(1.5), Color("black")]);


    fg.set_terminal("pngcairo", &format!("{}.png", name));
    fg.show();
}

fn main() {
    let n = 11;
    let pts = linspace(-1.0, 1.0, 301);
    let u_grid = linspace(-1.0, 1.0, n); // uniform grid
    let c_grid = chebyshev_knots(n);                 // Chebyshev grid
    let mut u_base = Vec::with_capacity(pts.len());
    let mut c_base = Vec::with_capacity(pts.len());
    for x in pts.iter() {
        u_base.push(polynomial_error(*x, &u_grid));
        c_base.push(polynomial_error(*x, &c_grid));
    }

    plot("Uniform grid"  , &pts, &u_base);
    plot("Chebyshev grid", &pts, &c_base);
}



