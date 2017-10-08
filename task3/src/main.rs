extern crate gnuplot;
extern crate la;
extern crate nalgebra as na;
extern crate interp_util;

use na::core::{DMatrix, MatrixVec, Dynamic};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::ops::IndexMut;
use gnuplot::*;
use interp_util::*;
use std::f64;

fn create_col(data: Vec<f64>) -> DMatrix<f64> {
    DMatrix::from_data(MatrixVec::new(Dynamic::new(data.len()), Dynamic::new(1), data))
}

fn f_col(col: &DMatrix<f64>, fun: &Fn(f64) -> f64) -> DMatrix<f64> {
    col.map(fun)
    //create_col(col.iter().cloned().map(fun).collect())
}

fn prod(lhs: &DMatrix<f64>, rhs: &DMatrix<f64>) -> f64 {
    lhs.component_mul(rhs).iter().sum()
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
        .points(x2, y2, &[Caption(name2), PointSymbol('x'), PointSize(1.5), Color("green")]);


    fg.set_terminal("pngcairo", &format!("{}.png", plot_name));
    fg.show();
}

fn main() {
    let xs : &[f64]  = &[0.0, 1.0, 2.0, 3.0, 4.0];
    let ys : &[f64]  = &[0.0, 1.0, -2.0, 3.0, -4.0];

    let mut resv : Vec<f64> = Vec::new();
    let mut rows : Vec<Vec<f64>> = Vec::new();

    for (x, y) in xs.iter().zip(ys.iter()) {
        resv.push(*y);
        let mut r : Vec<f64> = Vec::new();

        for i in 0..2 + xs.len() {
            r.push((*x).powi(i as i32));
        }

        println!("{:?}", r);
        rows.push(r);
    }


    {
        let mut r = Vec::new();
        resv.push(0.0);
        r.push(0.0);
        r.push(1.0);
        for i in 0..xs.len() {
            r.push(0.0);
        }
        rows.push(r);
    }
    {
        let mut r = Vec::new();
        resv.push(0.0);
        r.push(0.0);
        r.push(1.0);
        let x = xs.last().unwrap();
        for i in 0..xs.len() {
            r.push(((i + 2) as f64) * x.powi((i + 1) as i32));
        }
        rows.push(r);
    }


    let mut g_mat = DMatrix::identity_generic(Dynamic::new(2 + xs.len()), Dynamic::new(2 + xs.len()));
    
    for i in 0..rows.len() {
        for j in 0..rows[i].len() {
            *g_mat.index_mut((j, i)) = rows[i][j];
        }
    }

    let xe: DMatrix<f64> = create_col(resv);

    let g_mat2 = la::Matrix::new(g_mat.nrows(), g_mat.ncols(), g_mat.iter().cloned().collect());
    let xe2 = la::Matrix::new(xe.nrows(), xe.ncols(), xe.iter().cloned().collect());
    let k = g_mat2.solve(&xe2).unwrap();
    println!("{:?}", k);
}

/*
fn main() {

    let mut in_x = Vec::new();
    let mut in_y = Vec::new();


    let f = File::open("data.csv").unwrap();
    let reader = BufReader::new(&f);
    for (i, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        println!("{}: {}", i, l);

        match i {
            0 => in_x = l.split(';').skip(1).map(|s| s.parse::<f64>().unwrap()).collect(),
            1 => in_y = l.split(';').skip(1).map(|s| s.parse::<f64>().unwrap()).collect(),
            _ => (),
        }
    }

    let t = create_col(in_x.clone());
    let x = create_col(in_y.clone());
    println!("{:?}", t);

    let f0 = |_: f64| { 1.0 };
    let f1 = |x: f64| x;
    let f2 = |x: f64| x*x;
    let f3 = |x: f64| x*x*x;
    let f4 = |x: f64| x*x*x*x;

    let funcs: Vec<&Fn(f64) -> f64> = vec!(&f0, &f1, &f2, &f3);//, &f4);
    let e = funcs.iter().map(|f| f_col(&t, f)).collect::<Vec<_>>();

    let mut g_mat = DMatrix::identity_generic(Dynamic::new(e.len()), Dynamic::new(e.len()));

    for i in 0..e.len() {
        for j in 0..e.len() {
            *g_mat.index_mut((i, j)) = prod(&e[i], &e[j]);
        }
    }
    
    let xe: DMatrix<f64> = create_col(e.iter().map(|ei| prod(ei, &x)).collect());

    let g_mat2 = la::Matrix::new(g_mat.nrows(), g_mat.ncols(), g_mat.iter().cloned().collect());
    let xe2 = la::Matrix::new(xe.nrows(), xe.ncols(), xe.iter().cloned().collect());
    let k = g_mat2.solve(&xe2).unwrap();
    println!("{:?}", k);

    let ft: &Fn(f64) -> f64 = &(|t| funcs.iter().map(|f| f(t)).zip(k.get_data().iter()).map(|(a, b)| a * b).sum());
    println!("{}", ft(2.0));

    let space = linspace(*in_x.first().unwrap(), *in_x.last().unwrap(), 300);
    let f_space = space.iter().cloned().map(ft).collect::<Vec<_>>();
    plot("Task3", "Min square fit", &space, &f_space, "Points", &in_x, &in_y)
}
*/
