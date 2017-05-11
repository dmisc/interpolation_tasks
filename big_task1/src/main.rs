extern crate gnuplot;
extern crate interp_util;
extern crate la;

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
        self.c_begin * (x - self.t_begin).powi(3) + self.c_end * (self.t_end - x).powi(3) +
        self.l_begin * (x - self.t_begin) + self.l_end * (self.t_end - x)
    }

    fn calc_der(&self, x: f64) -> f64 {
        3.0 * self.c_begin * (x - self.t_begin).powi(2) -
        3.0 * self.c_end * (self.t_end - x).powi(2) + self.l_begin - self.l_end
    }
    fn calc_der2(&self, x: f64) -> f64 {
        6.0 * self.c_begin * (x - self.t_begin) + 6.0 * self.c_end * (self.t_end - x)
    }
}

#[derive(Debug)]
struct CubicSpline {
    sections: Vec<CubicSection>,
    section_bounds: Vec<f64>,
}

impl CubicSpline {
    fn calc(&self, x: f64) -> f64 {
        self.find_section(x).calc(x)
    }
    fn calc_der(&self, x: f64) -> f64 {
        self.find_section(x).calc_der(x)
    }
    fn calc_der2(&self, x: f64) -> f64 {
        self.find_section(x).calc_der2(x)
    }

    fn find_section(&self, x: f64) -> &CubicSection {
        for (i, &t) in self.section_bounds.iter().skip(1).enumerate() {
            if x < t {
                return &self.sections[i];
            }
        }

        &self.sections.last().unwrap()
    }
}

// a -- lower diagonal
// b -- middle
// c -- upper
// d -- result column
fn solve_tridiagonal(a: &[f64], b: &[f64], c: &[f64], d: &[f64]) -> Vec<f64> {
    println!("a {}, b {}, c {}, d {}", a.len(), b.len(), c.len(), d.len());
    let mut cm = Vec::new();
    let mut dm = Vec::new();
    cm.push(c[0] / b[0]);
    dm.push(d[0] / b[0]);
    for i in 1..c.len() {
        let tmp_c = c[i] / (b[i] - a[i - 1] * cm[i - 1]);
        cm.push(tmp_c);
    }
    for i in 1..b.len() {
        let tmp_d = (d[i] - a[i - 1] * dm[i - 1]) / (b[i] - a[i - 1] * cm[i - 1]);
        dm.push(tmp_d);
    }
    let mut res = Vec::new();
    res.push(*dm.last().unwrap());
    for i in 1..cm.len() + 1 {
        let tmp = dm[cm.len() - i] - cm[cm.len() - i] * res[i - 1];
        res.push(tmp);
    }
    res.reverse();

    res
}

fn create_cubic_spline_natural(pts: &[(f64, f64)]) -> CubicSpline {
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
        sections.push(CubicSection {
                          c_begin: z[i + 1] / (6.0 * h[i]),
                          c_end: z[i] / (6.0 * h[i]),
                          l_begin: y[i + 1] / h[i] - z[i + 1] * h[i] / 6.0,
                          l_end: y[i] / h[i] - h[i] * z[i] / 6.0,
                          t_begin: t[i],
                          t_end: t[i + 1],
                      })
    }

    CubicSpline {
        sections: sections,
        section_bounds: t,
    }
}

fn create_cubic_spline_clamped2(pts: &[(f64, f64)]) -> CubicSpline {
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

    v[0] = 1.5 * h[0] + 2.0 * h[1];
    *v.last_mut().unwrap() = 1.5 * h[h.len() - 2] + 2.0 * h[h.len() - 1];
    u[0] = u[0] - 3.0 * b[0];
    *u.last_mut().unwrap() = u[u.len() - 1] + 3.0 * b.last().unwrap();

    let sliced_h = &h[1..h.len() - 1];
    let res = solve_tridiagonal(sliced_h, &v, sliced_h, &u);
    let mut z = Vec::new();
    z.push(0.5 * (6.0 * b[0] / h[0] - res[0]));
    z.extend_from_slice(&res);
    z.push(-0.5 * (6.0 * b.last().unwrap() / h.last().unwrap() + res.last().unwrap()));

    let mut sections = Vec::new();

    for i in 0..z.len() - 1 {
        sections.push(CubicSection {
                          c_begin: z[i + 1] / (6.0 * h[i]),
                          c_end: z[i] / (6.0 * h[i]),
                          l_begin: y[i + 1] / h[i] - z[i + 1] * h[i] / 6.0,
                          l_end: y[i] / h[i] - h[i] * z[i] / 6.0,
                          t_begin: t[i],
                          t_end: t[i + 1],
                      })
    }

    CubicSpline {
        sections: sections,
        section_bounds: t,
    }
}

fn create_cubic_spline_clamped(pts: &[(f64, f64)]) -> CubicSpline {
    let der0 = -100.0;
    let der1 = 200.0;

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

    let mut real_v = Vec::new();
    real_v.push(2.0);
    real_v.extend_from_slice(&v);
    real_v.push(2.0);

    let mut upper_h = Vec::new();
    upper_h.push(1.0);
    //upper_h.extend_from_slice(&h);
    upper_h.extend_from_slice(&h[1..h.len() - 1]);
    upper_h.push(1.0);

    let mut lower_h = Vec::new();
    lower_h.push(1.0);
    lower_h.extend_from_slice(&h[1..h.len() - 1]);
    lower_h.push(1.0);

    let mut real_u = Vec::new();
    real_u.push(6.0 * (b[0] - der0) / h[0]);
    real_u.extend_from_slice(&u);
    real_u.push(6.0 * (der1 - b.last().unwrap()) / h.last().unwrap());

    let res = solve_tridiagonal(&lower_h, &real_v, &upper_h, &real_u);
    let mut z = Vec::new();
    //z.push(0.5 * (6.0 * b[0] / h[0] - res[0]));
    z.extend_from_slice(&res);
    //z.push(-0.5 * (6.0 * b.last().unwrap() / h.last().unwrap() + res.last().unwrap()));

    let mut sections = Vec::new();

    for i in 0..z.len() - 1 {
        sections.push(CubicSection {
                          c_begin: z[i + 1] / (6.0 * h[i]),
                          c_end: z[i] / (6.0 * h[i]),
                          l_begin: y[i + 1] / h[i] - z[i + 1] * h[i] / 6.0,
                          l_end: y[i] / h[i] - h[i] * z[i] / 6.0,
                          t_begin: t[i],
                          t_end: t[i + 1],
                      })
    }

    CubicSpline {
        sections: sections,
        section_bounds: t,
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

fn calc_lagrange_polynomial_der(at_x: f64, pts: &[(f64, f64)]) -> f64 {
    let lj = |j: usize| -> f64 {
        let xj = pts[j].0;
        pts.iter()
            .map(|&(x, _)| if x == xj { 1.0 } else { (at_x - x) / (xj - x) })
            .product()
    };
    let lj_der = |j: usize| -> f64 {
        let xj = pts[j].0;
        let a: f64 = lj(j);
        let b: f64 = pts.iter()
            .map(|&(x, _)| if x == xj { 0.0 } else { 1.0 / (at_x - x) })
            .sum();

        return a * b;
    };
    pts.iter()
        .map(|&pt| pt.1)
        .zip((0..pts.len()).map(lj_der))
        .map(|(yj, lj)| yj * lj)
        .sum()
}

fn calc_lagrange_polynomial_der2(at_x: f64, pts: &[(f64, f64)]) -> f64 {
    let prod_exc = |i: usize, l: usize, m: usize| -> f64 {
        pts.iter().enumerate()
            .map(|(id, &(x, _))| if id == i || id == l || id == m { 1.0 } else { (at_x - x) / (pts[i].0 - x) })
            .product()
    };
    let sum_exc = |i: usize, l: usize| -> f64 {
        pts.iter().enumerate()
            .map(|(id, &(x, _))| if id == i || id == l { 0.0 } else { prod_exc(i, l, id) / (pts[i].0 - x) })
            .sum()
    };
    let lj_der2 = |j: usize| -> f64 {
        pts.iter().enumerate()
            .map(|(id, &(x, _))| if id == j { 0.0 } else { sum_exc(j, id) / (pts[j].0 - x) })
            .sum()
    };
    pts.iter()
        .map(|&pt| pt.1)
        .zip((0..pts.len()).map(lj_der2))
        .map(|(yj, lj)| yj * lj)
        .sum()
}

fn main() {
    //let x_str = "-2  -1.68421    -1.36842    -1.05263    -0.73684    -0.42105    -0.10526    0.210526    0.526316    0.842105    1.157895    1.473684    1.789474    2.105263    2.421053    2.736842    3.052632    3.368421    3.684211    4";
    //let y_str = "6.880111    5.296874    3.96331 2.891384    2.089794    1.538613    1.148618    0.810363    0.551963    0.492903    0.696817    1.169522    1.899773    2.877061    4.098098    5.577943    7.365993    9.522361    12.00553    14.59995";

    let x_str = "0 1 2 3 4";
    let y_str = "0 3 1 2 2";

    let xs = x_str
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(f64::from_str)
        .map(Result::unwrap)
        .collect::<Vec<f64>>();
    let ys = y_str
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(f64::from_str)
        .map(Result::unwrap)
        .collect::<Vec<f64>>();
    let data = xs.iter()
        .cloned()
        .zip(ys.iter().cloned())
        .collect::<Vec<_>>();

    let pol_x = linspace(xs[0], xs[xs.len() - 1], 200);
    let pol_y = pol_x
        .iter()
        .map(|x| calc_lagrange_polynomial(*x, &data))
        .collect::<Vec<_>>();
    let pol_der_y = pol_x
        .iter()
        .map(|x| calc_lagrange_polynomial_der(*x, &data))
        .collect::<Vec<_>>();
    let pol_der2_y = pol_x
        .iter()
        .map(|x| calc_lagrange_polynomial_der2(*x, &data))
        .collect::<Vec<_>>();

    let spline = create_cubic_spline_natural(&data);
    let cub_y = pol_x
        .iter()
        .map(|x| spline.calc(*x))
        .collect::<Vec<_>>();
    let cub_der_y = pol_x
        .iter()
        .map(|x| spline.calc_der(*x))
        .collect::<Vec<_>>();
    let cub_der2_y = pol_x
        .iter()
        .map(|x| spline.calc_der2(*x))
        .collect::<Vec<_>>();

    let spline = create_cubic_spline_clamped(&data);
    let clamped_cub_y = pol_x
        .iter()
        .map(|x| spline.calc(*x))
        .collect::<Vec<_>>();
    let clamped_cub_der_y = pol_x
        .iter()
        .map(|x| spline.calc_der(*x))
        .collect::<Vec<_>>();
    let clamped_cub_der2_y = pol_x
        .iter()
        .map(|x| spline.calc_der2(*x))
        .collect::<Vec<_>>();

    plot_line_and_points("cubic"     , "Natural cubic spline"                  , &xs, &ys, &pol_x, &cub_y     );
    plot_line_and_points("cubic_der" , "Natural cubic spline derivative"       , &xs, &ys, &pol_x, &cub_der_y );
    plot_line_and_points("cubic_der2", "Natural cubic spline second derivative", &xs, &ys, &pol_x, &cub_der2_y);

    plot_line_and_points("clamped_cubic"     , "Clamped cubic spline"                  , &xs, &ys, &pol_x, &clamped_cub_y     );
    plot_line_and_points("clamped_cubic_der" , "Clamped cubic spline derivative"       , &xs, &ys, &pol_x, &clamped_cub_der_y );
    plot_line_and_points("clamped_cubic_der2", "Clamped cubic spline second derivative", &xs, &ys, &pol_x, &clamped_cub_der2_y);

    plot_line_and_points("lagrange"     , "Lagrange poly"                  , &xs, &ys, &pol_x, &pol_y     );
    plot_line_and_points("lagrange_der" , "Lagrange poly derivative"       , &xs, &ys, &pol_x, &pol_der_y );
    plot_line_and_points("lagrange_der2", "Lagrange poly second derivative", &xs, &ys, &pol_x, &pol_der2_y);
}

fn plot_line_and_points(plot_name: &str, line_caption: &str, pt_x: &[f64], pt_y: &[f64], line_x: &[f64], line_y: &[f64]) {
    let mut fg = Figure::new();
    fg.axes2d()
        .set_size(1.0, 1.0)
        .set_x_ticks(Some((Auto, 1)), &[Mirror(false)], &[])
        .set_y_ticks(Some((Auto, 1)), &[Mirror(false)], &[])
        .lines(line_x,
               line_y,
               &[Caption(line_caption), LineWidth(1.5), Color("red")])
        .points(pt_x, pt_y, &[Caption("Points"), PointSize(1.0), Color("blue")]);

    fg.set_terminal("png", &format!("{}.png", plot_name));
    fg.show();
}

// Reference:
//http://www.physics.arizona.edu/~restrepo/475A/Notes/sourcea-/node35.html
//http://cis.poly.edu/~mleung/CS3734/s03/ch07/cubicSpline.pdf
//https://www.math.ntnu.no/emner/TMA4215/2008h/cubicsplines.pdf
