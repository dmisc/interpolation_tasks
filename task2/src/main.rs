extern crate gnuplot;
extern crate interp_util;

use std::f64;

use gnuplot::*;
use interp_util::*;

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
        let mut tmp = 0.0;
        for j in 0..pts.len() {
            let mut tmp2 = 1.0;
            for k in 0..pts.len() {
                if k != j {
                    tmp2 *= x0 - pts[k];
                }
            }
            tmp += tmp2;
        }
        let mut tmp3 = 1.0;
        for j in 0..pts.len() {
            if j != i {
                tmp3 *= pts[i] - pts[j];
            }
        }

        res += (tmp / tmp3).abs();
    }

    res
}

fn chebyshev_knots(k: usize) -> Vec<f64> {
    let mut res = Vec::new();

    for m in 0..k {
        res.push((f64::consts::PI * ((2 * m + 1) as f64) / ((2 * k) as f64)).cos());
    }

    res
}

fn plot(plot_name: &str, name1: &str, x1: &[f64], y1: &[f64], name2: &str, x2: &[f64], y2: &[f64]) {
    let mut fg = Figure::new();

    fg.axes2d()
        .set_size(0.75, 1.0)
        .set_title(plot_name, &[])
        .set_x_ticks(Some((Fix(1.0), 1)), &[Mirror(false)], &[])
        .set_y_ticks(Some((Auto, 1)), &[Mirror(false)], &[])
        .set_legend(Graph(1.0), Graph(0.5), &[Placement(AlignLeft, AlignCenter)], &[TextAlign(AlignRight)])
        .set_border(true, &[Left, Bottom], &[LineWidth(2.0)])
        .set_x_label("Abscissa", &[])
        .set_y_label("Ordinate", &[])
        .lines(x1, y1, &[Caption(name1), LineWidth(1.5), Color("red")])
        .lines(x2, y2, &[Caption(name2), LineWidth(1.5), Color("green")]);


    fg.set_terminal("pngcairo", &format!("{}.png", plot_name));
    fg.show();
}

fn main() {
    let n = 11;
    let pts = linspace(-1.0, 1.0, 301);
    let u_grid = linspace(-1.0, 1.0, n); // uniform grid
    let c_grid = chebyshev_knots(n);                 // Chebyshev grid
    let mut u_base = Vec::with_capacity(pts.len());
    let mut c_base = Vec::with_capacity(pts.len());
    let mut u_base_der = Vec::with_capacity(pts.len());
    let mut c_base_der = Vec::with_capacity(pts.len());
    for x in pts.iter() {
        u_base.push(polynomial_error(*x, &u_grid));
        c_base.push(polynomial_error(*x, &c_grid));
        u_base_der.push(polynomial_derivative_error(*x, &u_grid));
        c_base_der.push(polynomial_derivative_error(*x, &c_grid));
    }

    plot("Px"  , "Uniform grid", &pts, &u_base, "Chebyshev grid", &pts, &c_base);
    plot("Px.der"  , "Uniform grid", &pts, &u_base_der, "Chebyshev grid", &pts, &c_base_der);
}



