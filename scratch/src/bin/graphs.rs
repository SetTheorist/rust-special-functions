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

use sf_impl::{
  agm,
  airy,
  algorithm,
  basic,
  bessel,
  complex,
  data,
  dawson,
  debye,
  dual,
  ellint,
  erf,
  exp,
  expint,
  float,
  gamma,
  hypergeom,
  jacobi,
  kahan,
  lambert,
  log,
  numbers,
  orthopoly,
  pcf,
  poly,
  polylog,
  real,
  sievert,
  solve,
  theta,
  traits,
  trig,
  twin,
  util,
  wide,
  zeta,
};

//use crate::kahan::{*};
//use crate::num::complex::{Complex};
use crate::algorithm::*;
use crate::bessel::*;
use crate::complex::*;
use crate::dawson::*;
use crate::dual::*;
use crate::ellint::*;
use crate::erf::{*};
use crate::exp::*;
use crate::gamma::*;
use crate::algorithm::integration::Integrator;
use crate::log::*;
use crate::numbers::*;
use crate::orthopoly::chebyshev_t::*;
use crate::orthopoly::*;
use crate::poly::*;
use crate::polylog::*;
use crate::real::*;
use crate::theta::*;
use crate::traits::*;
use crate::trig::*;
use crate::wide::{Wide};
use crate::zeta::*;

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
  if true {
    test_airy();
    test_dilog();
    test_erf();
    test_gamma();
  }
}

use std::io::{self,BufRead};
fn test_gamma() {
  let file = std::fs::File::open("./data/gamma.real.csv").unwrap();
  let mut t = Vec::new();
  for line in io::BufReader::new(file).lines() {
    if let Ok(line) = line {
      let v = line.split(",").collect::<Vec<_>>();
      let x : f64 = v[0].parse().unwrap();
      let fx : f64 = v[1].parse().unwrap();
      t.push((x,fx));
    }
  }
  let t = t.into_iter().map(|(x,fx)|{
    let apx = sf_gamma(r64(x)).0;(x,rel(fx,apx))})
    .collect::<Vec<_>>();

  let lo = -17.0;
  let hi = 0.0;
  let dat = plotlib::repr::Plot::new(t)
    .point_style(
      plotlib::style::PointStyle::new()
      .marker(plotlib::style::PointMarker::Circle)
      .colour("#113355")
      .size(0.5));
  let v = plotlib::view::ContinuousView::new()  
    .add(dat)
    .y_range(lo, hi)
    .x_label("x")
    .y_label("Gamma(x) relative error");
  plotlib::page::Page::single(&v).save("gamma_real_error.svg").expect("saving svg");
}
fn test_airy() {
  let file = std::fs::File::open("./data/airy.real.csv").unwrap();
  let mut t = Vec::new();
  for line in io::BufReader::new(file).lines() {
    if let Ok(line) = line {
      let v = line.split(",").collect::<Vec<_>>();
      let x : f64 = v[0].parse().unwrap();
      let ax : f64 = v[1].parse().unwrap();
      let bx : f64 = v[2].parse().unwrap();
      t.push((x,ax,bx));
    }
  }

  let lo = -17.0;
  let hi = 0.0;
  let ta = t.iter().map(|&(x,ax,_)|{
    let apx = airy::sf_airy_ai(r64(x)).0;(x,rel(ax,apx))})
    .collect::<Vec<_>>();
  let dat_a = plotlib::repr::Plot::new(ta)
    .point_style(
      plotlib::style::PointStyle::new()
      .marker(plotlib::style::PointMarker::Circle)
      .colour("#1133EE")
      .size(0.5));

  let tb = t.iter().map(|&(x,_,bx)|{
    let apx = airy::sf_airy_bi(r64(x)).0;(x,rel(bx,apx))})
    .collect::<Vec<_>>();
  let dat_b = plotlib::repr::Plot::new(tb)
    .point_style(
      plotlib::style::PointStyle::new()
      .marker(plotlib::style::PointMarker::Circle)
      .colour("#EE3311")
      .size(0.5));

  let ti = t.iter().map(|&(x,ax,_)|{
    let apx = airy::impls::airy_series(ι(x):wide::Wide).0.hi();(x,rel(ax,apx))})
    .collect::<Vec<_>>();
  let dat_i = plotlib::repr::Plot::new(ti)
    .point_style(
      plotlib::style::PointStyle::new()
      .marker(plotlib::style::PointMarker::Circle)
      .colour("#33EE11")
      .size(0.5));

  let tj = t.iter().map(|&(x,ax,_)|{
    let apx = airy::impls::ai_integ_pos__wide(wide::Wide(x,0.0)).0;(x,rel(ax,apx))})
    .collect::<Vec<_>>();
  let dat_j = plotlib::repr::Plot::new(tj)
    .point_style(
      plotlib::style::PointStyle::new()
      .marker(plotlib::style::PointMarker::Circle)
      .colour("#119999")
      .size(0.75));

  let tk = t.iter().map(|&(x,ax,_)|{
    let apx = airy::impls::ai_asympt_pos(wide::Wide(x,0.0)).0;(x,rel(ax,apx))})
    .collect::<Vec<_>>();
  let dat_k = plotlib::repr::Plot::new(tk)
    .point_style(
      plotlib::style::PointStyle::new()
      .marker(plotlib::style::PointMarker::Circle)
      .colour("#DDDD11")
      .size(0.75));

  let v = plotlib::view::ContinuousView::new()  
    .add(dat_a)
    .add(dat_b)
    .add(dat_i)
    .add(dat_j)
    .add(dat_k)
    .y_range(lo, hi)
    .x_label("x")
    .y_label("Airy Ai(x) (blue) &amp; Bi(x) (red) relative error");
  plotlib::page::Page::single(&v).save("airy_real_error.svg").expect("saving svg");
}
fn test_dilog() {
  let file = std::fs::File::open("./data/dilog.real.csv").unwrap();
  let mut t = Vec::new();
  for line in io::BufReader::new(file).lines() {
    if let Ok(line) = line {
      let v = line.split(",").collect::<Vec<_>>();
      let x : f64 = v[0].parse().unwrap();
      let fx : f64 = v[1].parse().unwrap();
      t.push((x,fx));
    }
  }
  let t = t.into_iter().map(|(x,fx)|{
    let apx = sf_dilog(r64(x)).0;(x,rel(fx,apx))})
    .collect::<Vec<_>>();

  let lo = -17.0;
  let hi = 0.0;
  let dat = plotlib::repr::Plot::new(t)
    .point_style(
      plotlib::style::PointStyle::new()
      .marker(plotlib::style::PointMarker::Circle)
      .colour("#113355")
      .size(0.5));
  let v = plotlib::view::ContinuousView::new()  
    .add(dat)
    .y_range(lo, hi)
    .x_label("x")
    .y_label("DiLog(x) relative error");
  plotlib::page::Page::single(&v).save("dilog_real_error.svg").expect("saving svg");
}
fn test_erf() {
  let file = std::fs::File::open("./data/erf.real.csv").unwrap();
  let mut t = Vec::new();
  for line in io::BufReader::new(file).lines() {
    if let Ok(line) = line {
      let v = line.split(",").collect::<Vec<_>>();
      let x : f64 = v[0].parse().unwrap();
      let fx : f64 = v[1].parse().unwrap();
      t.push((x,fx));
    }
  }
  let t = t.into_iter().map(|(x,fx)|{
    let apx = sf_erf(r64(x)).0;(x,rel(fx,apx))})
    .collect::<Vec<_>>();

  let lo = -17.0;
  let hi = 0.0;
  let dat = plotlib::repr::Plot::new(t)
    .point_style(
      plotlib::style::PointStyle::new()
      .marker(plotlib::style::PointMarker::Circle)
      .colour("#113355")
      .size(0.5));
  let v = plotlib::view::ContinuousView::new()  
    .add(dat)
    .y_range(lo, hi)
    .x_label("x")
    .y_label("Erf(x) relative error");
  plotlib::page::Page::single(&v).save("erf_real_error.svg").expect("saving svg");
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