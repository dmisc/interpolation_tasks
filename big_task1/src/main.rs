extern crate gnuplot;
extern crate interp_util;

use std::f64;
use std::str::FromStr;

use gnuplot::*;
use interp_util::*;

#[derive(Debug)]
struct CubicSection {
    c_begin: f64,
    c_end: f64,

    l_begin: f64,
    l_end: f64,

    t_begin: f64,
    t_end: f64,
}

impl CubicSection {
    fn calc(&self, x: f64) -> f64 {
        self.c_begin * (x - self.t_begin).powi(3) + 
            self.c_end * (self.t_end - x).powi(3) + 
            self.l_begin * (x - self.t_begin) + 
            self.l_end   * (self.t_end - x)
    }
}

#[derive(Debug)]
struct CubicSpline {
    sections: Vec<CubicSection>,
    section_bounds: Vec<f64>,
}

impl CubicSpline {
    fn calc(&self, x: f64) -> f64 {
        for (i, &t) in self.section_bounds.iter().skip(1).enumerate() {
            if x < t {
                return self.sections[i].calc(x);
            }
        }

        self.sections.last().unwrap().calc(x)
    }
}

// a -- lower diagonal
// b -- middle
// c -- upper
// d -- result column
fn solve_tridiagonal(a: &[f64], b: &[f64], c: &[f64], d: &[f64]) -> Vec<f64> {
    let mut cm = Vec::new();
    let mut dm = Vec::new();
    cm.push(c[0] / b[0]);
    dm.push(d[0] / b[0]);
    for i in 1..c.len() {
        let tmp_c = c[i] / (b[i] - a[i] * cm[i - 1]);
        cm.push(tmp_c);

        let tmp_d = (d[i] - a[i] * dm[i - 1]) / (b[i] - a[i] * cm[i - 1]);
        dm.push(tmp_d);
    }
    let mut res = Vec::new();
    res.push(*dm.last().unwrap());
    for i in 1..1 + dm.len() {
        let tmp = dm[dm.len() - i] - cm[dm.len() - i] * res[i - 1];
        res.push(tmp);
    }
    res.reverse();

    res
}

fn create_cubic_spline(pts: &[(f64, f64)]) -> CubicSpline {
    let y = pts.iter().map(|&(_, y)| y).collect::<Vec<_>>();
    let t = pts.iter().map(|&(t, _)| t).collect::<Vec<_>>();

    let n = pts.len();
    let mut h = Vec::new();
    let mut b = Vec::new();
    let mut v = Vec::new();
    let mut u = Vec::new();

    for i in 0..n - 1 {
        h.push(t[i + 1] - t[i]);
        b.push((y[i + 1] - y[i]) / h.last().unwrap());
    }
    for i in 1..n - 1 {
        v.push(2.0 * (h[i - 1] + h[i]));
        u.push(6.0 * (b[i] - b[i - 1]));
    }

    let sliced_h = &h[1..h.len() - 1];
    let res = solve_tridiagonal(sliced_h, &v, sliced_h, &u);
    let mut z = Vec::new();
    z.push(0.0);
    z.extend_from_slice(&res);
    z.push(0.0);

    let mut sections = Vec::new();

    for i in 0..z.len() - 1 {
        sections.push(
            CubicSection {
                c_begin: z[i+1] / (6.0 * h[i]),
                c_end  : z[i  ] / (6.0 * h[i]),
                l_begin: y[i + 1] / h[i] - z[i + 1] * h[i] / 6.0,
                l_end  : y[i    ] / h[i] - h[i] * z[i] / 6.0,
                t_begin: t[i],
                t_end: t[i + 1],
            }
        )
    }

    CubicSpline {
        sections: sections,
        section_bounds: t
    }
}

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
    let x_str = "-2  -1.68421    -1.36842    -1.05263    -0.73684    -0.42105    -0.10526    0.210526    0.526316    0.842105    1.157895    1.473684    1.789474    2.105263    2.421053    2.736842    3.052632    3.368421    3.684211    4";
    let y_str = "6.880111    5.296874    3.96331 2.891384    2.089794    1.538613    1.148618    0.810363    0.551963    0.492903    0.696817    1.169522    1.899773    2.877061    4.098098    5.577943    7.365993    9.522361    12.00553    14.59995";

    let xs = x_str.split(' ').filter(|s| !s.is_empty()).map(f64::from_str).map(Result::unwrap).collect::<Vec<f64>>();
    let ys = y_str.split(' ').filter(|s| !s.is_empty()).map(f64::from_str).map(Result::unwrap).collect::<Vec<f64>>();
    let data = xs.iter().cloned().zip(ys.iter().cloned()).collect::<Vec<_>>();

    let pol_x = linspace(xs[0] - 0.1, xs[xs.len() - 1] + 0.1, 500);
    let pol_y = pol_x.iter().map(|x| { calc_lagrange_polynomial(*x, &data) }).collect::<Vec<_>>();

    let spline = create_cubic_spline(&data);
    let cub_y = pol_x.iter().map(|x| { spline.calc(*x) }).collect::<Vec<_>>();

    let mut fg = Figure::new();
    fg.axes2d()
        .set_size(1.0, 1.0)
        .set_legend(Graph(0.5), Graph(1.0), &[], &[])
        .set_x_ticks(Some((Auto, 1)), &[Mirror(false)], &[])
        .set_y_ticks(Some((Auto, 1)), &[Mirror(false)], &[])
        .lines(&pol_x, &pol_y,
            &[Caption("Lagrange poly"), LineWidth(1.5), Color("green")])
        .lines(&pol_x, &cub_y,
            &[Caption("Cubic spline"), LineWidth(1.5), Color("blue")])
        .points(&xs, &ys, &[Caption("Points"), PointSize(1.0), Color("red")]);

    //fg.set_terminal("svg", "result.svg");
    fg.set_terminal("png", "result.png");
    fg.show();
    fg.echo_to_file("plot.txt");
}
