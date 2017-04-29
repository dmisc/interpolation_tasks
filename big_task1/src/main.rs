extern crate gnuplot;
extern crate interp_util;

use std::f64;

use gnuplot::*;
use interp_util::*;


fn calc_lagrange_polynomial(at_x: f64, pts: &[(f64, f64)]) -> f64 {
    let lj = |j: usize| -> f64 {
        let xj = pts[j].0;
        pts.iter()
            .map(|&(x, _)| if x == xj { 1.0 } else { (at_x - x) / (xj - x) })
            .product()
    };
    pts.iter()
        .map(|&pt| pt.1)
        .zip((0..pts.len()).map(lj))
        .map(|(yj, lj)| yj * lj)
        .sum()
}

fn main() {
    println!("Hello, world!");
    let mut data = vec![(1.0, 15.0), (1.5, -10.0), (2.0, 11.0), (2.5, -40.0), (3.0, 34.0)];
    let res = calc_lagrange_polynomial(0.3, &data);
    println!("Res: {}", res);
    let xs = data.iter().map(|&pt| pt.0).collect::<Vec<_>>();
    let ys = data.iter().map(|&pt| pt.1).collect::<Vec<_>>();

    let pol_x = linspace(0.8, 3.2, 100);
    let pol_y = pol_x.iter().map(|x| { calc_lagrange_polynomial(*x, &data) }).collect::<Vec<_>>();

    let mut fg = Figure::new();
    fg.axes2d()
        .set_size(1.0, 1.0)
        .set_legend(Graph(0.5), Graph(1.0), &[], &[])
        .set_x_ticks(Some((Auto, 1)), &[Mirror(false)], &[])
        .set_y_ticks(Some((Auto, 1)), &[Mirror(false)], &[])
        .lines(&pol_x, &pol_y,
            &[Caption("Lagrange poly"), LineWidth(1.5), Color("green")])
        .points(&xs, &ys, &[Caption("Points"), PointSize(1.0), Color("red")]);

    fg.set_terminal("pngcairo", "lagrange.png");
    fg.show();
}
