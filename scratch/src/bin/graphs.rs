#![allow(clippy::comparison_chain)]
#![allow(clippy::eq_op)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::float_cmp)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::suspicious_arithmetic_impl)]
#![allow(clippy::suspicious_op_assign_impl)]
#![allow(clippy::wrong_self_convention)]
#![allow(confusable_idents)]
#![allow(dead_code)]
#![allow(mixed_script_confusables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![feature(bench_black_box)]
#![feature(bigint_helper_methods)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(int_log)]
#![feature(trait_alias)]
#![feature(type_ascription)]
#![feature(unchecked_math)]
//#![feature(marker_trait_attr)] // #[marker]
//#![feature(never_type)]
//#![feature(optimize_attribute)] // [#optimize(speed)]
//#![feature(specialization)]

use std::io::{self,BufRead};

use sf_impl::{
  agm, airy, algorithm, basic, bessel,
  complex, data, dawson, debye, dual,
  ellint, erf, exp, expint, float,
  gamma, hypergeom, jacobi, kahan, lambert,
  log, numbers, orthopoly, pcf, poly,
  polylog, real, sievert, solve, theta,
  traits, trig, twin, util, wide,
  zeta,
};

use crate::complex::*;
use crate::real::*;
use crate::traits::*;
use crate::wide::{Wide};

fn rel(ex: f64, ap: f64) -> f64 {
  let ε = f64::EPSILON;
  let l10 = 10.0_f64.ln();
  let res = if ex == ap {
    ε.ln()/l10 - 1.0
  } else {
    // TODO
    //((ap - ex).abs() / (ε*ε + ex.abs())).ln() / l10
    ((ap - ex).abs() / ex.abs()).ln() / l10
  };
  if !res.is_finite() {0.0} else {res}
}

pub fn main() {
  gen_graph_r64(airy::sf_airy_ai, "./data/airy.real.csv", "#AA3355", 0, 1, "x", "Airy AI(x) rel.err.", "./accuracy/airy.ai.real.svg");
  gen_graph_r64(airy::sf_airy_bi, "./data/airy.real.csv", "#AA3355", 0, 2, "x", "Airy BI(x) rel.err.", "./accuracy/airy.bi.real.svg");
  gen_graph_r64(polylog::sf_dilog, "./data/dilog.real.csv", "#AA3355", 0, 1, "x", "Dilog(x) rel.err.", "./accuracy/dilog.real.svg");
  gen_graph_r64(erf::sf_erf, "./data/erf.real.csv", "#AA3355", 0, 1, "x", "Erf(x) rel.err.", "./accuracy/erf.real.svg");
  gen_graph_r64(gamma::sf_gamma, "./data/gamma.real.csv", "#AA3355", 0, 1, "x", "Gamma(x) rel.err.", "./accuracy/gamma.real.svg");
  gen_graph_r64(zeta::sf_zeta, "./data/zeta.real.csv", "#AA3355", 0, 1, "x", "Riemann zeta(x) rel.err.", "./accuracy/zeta.real.svg");
  gen_graph_r64(zeta::sf_zeta_m1, "./data/zeta.real.csv", "#AA3355", 0, 2, "x", "Riemann zeta(x)-1 rel.err.", "./accuracy/zeta_m1.real.svg");

  gen_graph_ortho_r64(orthopoly::chebyshev_t::ChebyshevT::new(), |x|ι(1), "x", "Chebyshev T", "./diagrams/chebyshev_t.svg", 6, (ι(-1),ι(1)), (ι(-1.1),ι(1.1)));
  gen_graph_ortho_r64(orthopoly::chebyshev_u::ChebyshevU::new(), |x|ι(1), "x", "Chebyshev U", "./diagrams/chebyshev_u.svg", 6, (ι(-1),ι(1)), (ι(-3.0),ι(3.0)));
  gen_graph_ortho_r64(orthopoly::legendre::Legendre::new(), |x|ι(1), "x", "Legendre", "./diagrams/legendre.svg", 6, (ι(-1),ι(1)), (ι(-1.1),ι(1.1)));

  gen_graph_ortho_r64(orthopoly::laguerre::Laguerre::new(ι(0.5)), |x|ι(1), "x", "Laguerre(0.5)", "./diagrams/laguerre_0.5.svg", 6, (ι(0),ι(5)), (ι(-3.0),ι(3.0)));
  gen_graph_ortho_r64(orthopoly::laguerre::Laguerre::new(ι(0.5)), |x|exp::sf_exp(-x)*sf_sqrt(x), "x", "Laguerre(0.5) (weighted)", "./diagrams/laguerre_0.5_weighted.svg", 6, (ι(0),ι(5)), (ι(-0.75),ι(0.75)));
  gen_graph_ortho_r64(orthopoly::laguerre::Laguerre::new(ι(1.5)), |x|ι(1), "x", "Laguerre(1.5)", "./diagrams/laguerre_1.5.svg", 6, (ι(0),ι(10)), (ι(-5.0),ι(5.0)));
  gen_graph_ortho_r64(orthopoly::laguerre::Laguerre::new(ι(1.5)), |x|exp::sf_exp(-x)*x*sf_sqrt(x), "x", "Laguerre(1.5) (weighted)", "./diagrams/laguerre_1.5_weighted.svg", 6, (ι(0),ι(10)), (ι(-1.0),ι(1.0)));

  gen_graph_ortho_r64(orthopoly::gegenbauer::Gegenbauer::new(ι(2.25)), |x|ι(1), "x", "Gegenbauer(2.25)", "./diagrams/gegenbauer_2.25.svg", 6, (ι(-1),ι(1)), (ι(-15.0),ι(15.0)));
  gen_graph_ortho_r64(orthopoly::gegenbauer::Gegenbauer::new(ι(2.25)), |x|(r64::one-x*x).pow(r64(2.25-0.5)), "x", "Gegenbauer(2.25) (weighted)", "./diagrams/gegenbauer_2.25_weighted.svg", 6, (ι(-1),ι(1)), (ι(-5.0),ι(5.0)));

  gen_graph_ortho_r64(orthopoly::hermite_h::HermiteH::new(), |x|ι(1), "x", "Hermite H", "./diagrams/hermite_h.svg", 4, (ι(-4),ι(4)), (ι(-15.0),ι(15.0)));
  gen_graph_ortho_r64(orthopoly::hermite_h::HermiteH::new(), |x|exp::sf_exp(-x*x), "x", "Hermite H (weighted)", "./diagrams/hermite_h_weighted.svg", 4, (ι(-4),ι(4)), (ι(-10.0),ι(15.0)));


  //let apx = airy::impls::airy_series(ι(x):wide::Wide).0.hi();(x,rel(ax,apx))})
  //let apx = airy::impls::ai_integ_pos__wide(wide::Wide(x,0.0)).0;(x,rel(ax,apx))})
  //let apx = airy::impls::ai_asympt_pos(wide::Wide(x,0.0)).0;(x,rel(ax,apx))})
}

fn gen_graph_r64<F:Fn(r64)->r64>(f:F, data:&str, color:&str, x_column:usize, f_column:usize, x_label:&str, y_label:&str, output:&str) {
  let mut t = Vec::new();
  let file = std::fs::File::open(data).unwrap();
  for line in io::BufReader::new(file).lines() {
    if let Ok(line) = line {
      let v = line.split(",").collect::<Vec<_>>();
      let x : f64 = v[x_column].parse().unwrap();
      let fx : f64 = v[f_column].parse().unwrap();
      t.push((x,fx));
    }
  }
  let t : Vec<_> = t.into_iter().map(|(x,fx)|{ let apx = f(r64(x)).0; (x,rel(fx,apx)) }).collect();

  let (lo, hi) = (-17.0, 0.0);
  let ps = plotlib::style::PointStyle::new()
      .marker(plotlib::style::PointMarker::Circle)
      .colour(color)
      .size(1.0);
  let dat = plotlib::repr::Plot::new(t).point_style(ps);
  let v = plotlib::view::ContinuousView::new()  
    .add(dat).y_range(lo, hi).x_label(x_label).y_label(y_label);
  plotlib::page::Page::single(&v).save(output).expect("saving svg");
}

fn gen_graph_ortho_r64<F:Fn(r64)->r64, OP:orthopoly::OrthogonalPolynomial<r64>>(op:OP, scale:F, x_label:&str, y_label:&str, output:&str,
    npoly:isize, xrange:(r64,r64), yrange:(r64,r64))
{
  let colors = [
    "black",
    "red",
    "blue",
    "green",
    "purple",
    "darkblue",
    "mediumorchid",
    "brown",
    "palegreen",
    "cornflowerblue",
    "indigo",
    "slategrey",
  ];
  let mut v = plotlib::view::ContinuousView::new()
      .y_range(yrange.0.0, yrange.1.0).x_label(x_label).y_label(y_label);
  v = v.add(
      plotlib::repr::Plot::new(vec![(xrange.0.0,0.0),(xrange.1.0,0.0)])
        .line_style(plotlib::style::LineStyle::new().colour("lightgray")));
  for n in 0..(npoly+1) {
    let mut t = Vec::new();
    for x in crate::util::Grid::new(xrange.0, xrange.1, 256) {
      let fx = op.value(n, x) * scale(x);
      t.push((x.0,fx.0));
    }
    let c = colors[n as usize];
    let ls = plotlib::style::LineStyle::new()
      .colour(c)
      .linejoin(plotlib::style::LineJoin::Round);
    let ps = plotlib::style::PointStyle::new()
      .marker(plotlib::style::PointMarker::Circle)
      .colour(c)
      .size(1.0);
    let dat = plotlib::repr::Plot::new(t)
      .line_style(ls)
      .point_style(ps);
    v = v.add(dat);
  }
  plotlib::page::Page::single(&v).save(output).expect("saving svg");
}

/*
extern crate plotlib;
fn doplots() {
  let lo = -10.0;
  let hi = 10.0;
  let f1 = plotlib::repr::Plot::from_function(
      |x|(rel(x.exp(),exp_cf(r64(x)).0)), lo, hi)
    .line_style(plotlib::style::LineStyle::new().colour("black"));
  let f2 = plotlib::repr::Plot::from_function(
      |x|(rel(x.exp(),sf_exp(x))), lo, hi)
    .line_style(plotlib::style::LineStyle::new().colour("red"));
  let f3 = plotlib::repr::Plot::from_function(
      |x|(rel(x.exp(),exp__powserk(x, 1.0))), lo, hi)
    .line_style(plotlib::style::LineStyle::new().colour("blue"));
  let v = plotlib::view::ContinuousView::new().add(f3).add(f2).add(f1).
    y_range(-18.0, -8.0);
  plotlib::page::Page::single(&v).save("plot1.svg").expect("saving svg");
}
*/
/*
extern crate plotters;
use plotters::prelude::*;
fn make_plot<C:Color>(fs:&[(C, &str, Vec<(f64,f64)>)], filename:&str, caption:&str,
    xrange:(f64,f64), yrange:(f64,f64))
  -> Result<(),Box<dyn std::error::Error>>
{
    //let root = BitMapBackend::new("0.png", (1280, 960)).into_drawing_area();
    //let root = SVGBackend::new(filename, (640, 480)).into_drawing_area();
    let root = SVGBackend::new(filename, (960, 720)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(xrange.0..xrange.1, yrange.0..yrange.1)?;
    chart.configure_mesh().draw()?;
    for (color,label,pts) in fs.iter() {
      chart.draw_series(LineSeries::new((*pts).iter().cloned(), &color,))?
        .label(*label);
        //.legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color));
    }
    /*
    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    */
    Ok(())
}
*/
/*
fn doplots() -> Result<(),Box<dyn std::error::Error>> {
    //let root = BitMapBackend::new("0.png", (1280, 960)).into_drawing_area();
    let root = SVGBackend::new("0.svg", (1280, 960)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=e^x", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-10f64..10f64, -18f64..-8f64)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
            (-1000..=1000).map(|x| x as f64/100.0).map(|x|
            (x, rel(x.exp(),exp__powserk(x,1.0)))
            ),
            &GREEN,
        ))?
        .label("power series")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));
    chart.draw_series(LineSeries::new(
            (-1000..=1000).map(|x| x as f64/100.0).map(|x|
            (x, rel(x.exp(),exp_cf(r64(x)).0))
            ),
            &RED,
        ))?
        .label("continued fraction")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    chart.draw_series(LineSeries::new(
            (-1000..=1000).map(|x| x as f64/100.0).map(|x|
            (x, rel(x.exp(),sf_exp(x)))
            ),
            &BLUE,
        ))?
        .label("range-reduction + power series")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
}
*/

  /*
    //if true { doplots(); }
    let ch : ChebyshevT<r64> = orthopoly::chebyshev_t::ChebyshevT::<r64>::new();
    if true {
      let fxrange = (-1.0,1.0);
      let num_points = 200; // 5000;
      let dx = (fxrange.1 - fxrange.0) / ((num_points - 1) as f64);
      let xrange = (-1.01,1.01);
      let yrange = (-1.05,1.05);
      make_plot(
        &(0..11)
          .map(|n|
            (BLACK, "T",
              (0..num_points).map(|ix| (ix as f64)*dx + fxrange.0)
                .map(|x|(x, ch.value(n,r64(x)).0)).collect::<Vec<_>>()))
          .collect::<Vec<_>>(),
        "chebyshev_t.svg", "Chebyshev T", xrange, yrange).unwrap();
      /*
      make_plot(&[
        (GREEN, "T1",
          (0..num_points).map(|ix| (ix as f64)*dx + xrange.0)
            .map(|x|(x, ch.value(1,r64(x)).0)).collect::<Vec<_>>()),
        (BLUE, "T2",
          (0..num_points).map(|ix| (ix as f64)*dx + xrange.0)
            .map(|x|(x, ch.value(2,r64(x)).0)).collect::<Vec<_>>()),
        (RED, "T3",
          (0..num_points).map(|ix| (ix as f64)*dx + xrange.0)
            .map(|x|(x, ch.value(3,r64(x)).0)).collect::<Vec<_>>()),
        (CYAN, "T4",
          (0..num_points).map(|ix| (ix as f64)*dx + xrange.0)
            .map(|x|(x, ch.value(4,r64(x)).0)).collect::<Vec<_>>()),
        (MAGENTA, "T5",
          (0..num_points).map(|ix| (ix as f64)*dx + xrange.0)
            .map(|x|(x, ch.value(5,r64(x)).0)).collect::<Vec<_>>()),
        (BLACK, "T11",
          (0..num_points).map(|ix| (ix as f64)*dx + xrange.0)
            .map(|x|(x, ch.value(11,r64(x)).0)).collect::<Vec<_>>()),
        ], "chebyshev_t.svg", "Chebyshev T", xrange, yrange).unwrap();
        */
    }
  */