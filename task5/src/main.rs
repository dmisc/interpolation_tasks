extern crate interp_util;
extern crate gnuplot;
extern crate rgsl;

use gnuplot::*;
use interp_util::*;
use rgsl::types::integration::IntegrationWorkspace;
use std::f64::consts::PI;
use std::thread;

struct FuncData {
    s: Box<Fn(f64) -> f64>,
    p: f64,
    n: i32,
}

fn fourier_a(x: f64, fd: &mut FuncData) -> f64 {
    (fd.s)(x) * (2.0 * PI * (fd.n as f64) * x / fd.p).cos()
}

fn fourier_b(x: f64, fd: &mut FuncData) -> f64 {
    (fd.s)(x) * (2.0 * PI * (fd.n as f64) * x / fd.p).sin()
}

fn calc_fourier(x: f64, a0: f64, an: &[f64], bn: &[f64], p: f64) -> f64 {
    a0 / 2.0 + 
        an.iter().enumerate().map(|(i, &ai)| ai * (2.0 * PI * ((1 + i) as f64) * x / p).cos()).sum::<f64>() +
        bn.iter().enumerate().map(|(i, &bi)| bi * (2.0 * PI * ((1 + i) as f64) * x / p).sin()).sum::<f64>() 
}

fn plot_fourier(a: f64, b: f64, a0: f64, an: &[f64], bn: &[f64], ref_fun: &Fn(f64) -> f64, name: &str) {
    let xs = linspace(a, b, 1000);
    let ref_y  = xs.iter().map(|&x| ref_fun(x)).collect::<Vec<_>>();
    let appr_y = xs.iter().map(|&x| calc_fourier(x, a0, an, bn, b - a)).collect::<Vec<_>>();

    plot_line_data(a, b, &xs, &ref_y, &xs, &appr_y, &format!("{}_{}", name, an.len()));
}

fn fourier_approximation(a: f64, b: f64, fun: Box<Fn(f64) -> f64>, ns: &[i32], name: &str) {
    let mut coef_a = Vec::new();
    let mut coef_b = Vec::new();
    let mut fd = FuncData {
        s: fun,
        p: b - a,
        n: 0,
    };

    let mut iw = IntegrationWorkspace::new(1000).unwrap();

    let a0 = {
        let mut res = 0.0;
        let mut err = 0.0;
        let status = iw.qag(
            fourier_a,
            &mut fd,
            a,
            b,
            0.001,
            0.1,
            1000,
            rgsl::GaussKonrodRule::Gauss15,
            &mut res,
            &mut err
        );
        assert!(status == rgsl::Value::Success);
        res * 2.0 / (b - a)
    };

    for i in 1.. {
        fd.n = i;
        let ai = {
            let mut res = 0.0;
            let mut err = 0.0;
            let status = iw.qag(
                fourier_a,
                &mut fd,
                a,
                b,
                0.05,
                0.2,
                1000,
                rgsl::GaussKonrodRule::Gauss31,
                &mut res,
                &mut err
            );
            assert!(status == rgsl::Value::Success);
            res * 2.0 / (b - a)
        };
        let bi = {
            let mut res = 0.0;
            let mut err = 0.0;
            let status = iw.qag(
                fourier_b,
                &mut fd,
                a,
                b,
                0.05,
                0.2,
                1000,
                rgsl::GaussKonrodRule::Gauss31,
                &mut res,
                &mut err
            );
            assert!(status == rgsl::Value::Success);
            res * 2.0 / (b - a)
        };
        coef_a.push(ai);
        coef_b.push(bi);

        if ns.iter().find(|&&n| n == i).is_some() {
            plot_fourier(a, b, a0, &coef_a, &coef_b, &*fd.s, name);
        }
        if i >= *ns.iter().last().unwrap() {
            break;
        }
    }
}

fn main() {
    let f1  = Box::new(|x: f64| -> f64 { 0.8 * x });
    let f2  = Box::new(|x: f64| -> f64 { x * x });
    let f3  = Box::new(|x: f64| -> f64 { x * x });
    let f4  = Box::new(|x: f64| -> f64 { x * x * x - 2.0 * x });
    let f5  = Box::new(|x: f64| -> f64 { (-x * x).exp() * (x * x + 1.0).ln() });

    //fourier_approximation(-1.0, 1.0, f1, &[6, 10, 13, 15, 17], "f1");

    fourier_approximation(-1.0, 1.0, f1, &[6, 10, 13, 17], "f1");
    fourier_approximation(-2.0, 2.0, f2, &[6, 10, 13], "f2");
    fourier_approximation(-1.0, 3.0, f3, &[6, 10, 14], "f3");
    fourier_approximation( 0.0,  PI, f4, &[6, 10, 14], "f4");
    fourier_approximation(-1.0, 1.0, f5, &[6, 10, 15], "f5");

    //let h1 = thread::spawn(|| { fourier_approximation(-1.0, 1.0, f1, &[6, 10, 13, 15, 17], "f1"); });
    //let h2 = thread::spawn(|| { fourier_approximation(-2.0, 2.0, f2, &[6, 10, 13, 15, 17], "f2"); });
    //let h3 = thread::spawn(|| { fourier_approximation(-1.0, 3.0, f3, &[6, 10, 13, 15, 17], "f3"); });
    //let h4 = thread::spawn(|| { fourier_approximation( 0.0,  PI, f4, &[6, 10, 13, 15, 17], "f4"); });
    //let h5 = thread::spawn(|| { fourier_approximation(-1.0, 1.0, f5, &[6, 10, 13, 15, 17], "f5"); });

    //let mut hs = vec![h1, h2, h3, h4, h5];
    //let h_len = hs.len();
    //for i in 0..h_len {
    //    hs.pop().unwrap().join();
    //}
}

fn plot_line_data(a: f64, b: f64, ref_x: &[f64], ref_y: &[f64], appr_x: &[f64], appr_y: &[f64], name: &str) {
    let mut fg = Figure::new();
    fg.axes2d()
        .set_size(1.0, 1.0)
        .set_legend(Graph(0.5), Graph(1.0), &[], &[])
        .set_x_ticks(Some((Auto, 1)), &[Mirror(false)], &[])
        .set_y_ticks(Some((Auto, 1)), &[Mirror(false)], &[])
        .lines(ref_x, ref_y, &[Caption("Reference function"), LineWidth(1.5), Color("green")])
        .lines(appr_x, appr_y, &[Caption("Fourier approximate"), LineWidth(1.0), Color("red")]);

    fg.set_terminal("pngcairo", &format!("{}.png", name));
    fg.show();
}
