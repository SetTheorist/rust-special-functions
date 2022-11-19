
fn Phi(z:f64) -> f64 { unimplemented!() }

pub fn bs_euro_upper(cp:bool, k:f64, t:f64, s:f64, r:f64, g:f64) -> f64 {
  if cp {
    let ert = (-r*t).exp();
    k*ert
  } else {
    let egt = (-g*t).exp();
    s*egt
  }
}

pub fn bs_euro_intrinsic(cp:bool, k:f64, t:f64, s:f64, r:f64, g:f64) -> f64 {
  let om = if cp {1.0} else {-1.0};
  let ert = (-r*t).exp();
  let egt = (-g*t).exp();
  0_f64.max(om*(egt*s - ert*k))
}

pub fn bs_euro_premium(cp:bool, k:f64, t:f64, s:f64, r:f64, g:f64, v:f64) -> f64 {
  let om = if cp {1.0} else {-1.0};
  let ert = (-r*t).exp();
  let egt = (-g*t).exp();
  let vqt = v * t.sqrt();
  let dp = ((s/k).ln() + (r-g)*t)/vqt + vqt/2.0;
  let dm = ((s/k).ln() + (r-g)*t)/vqt - vqt/2.0;
  om*(egt*s*Phi(om*dp) - ert*k*Phi(om*dm))
}

pub fn bs_euro_delta(cp:bool, k:f64, t:f64, s:f64, r:f64, g:f64, v:f64) -> f64 {
  let om = if cp {1.0} else {-1.0};
  let ert = (-r*t).exp();
  let egt = (-g*t).exp();
  let vqt = v * t.sqrt();
  let dp = ((s/k).ln() + (r-g)*t)/vqt + vqt/2.0;
  let dm = ((s/k).ln() + (r-g)*t)/vqt - vqt/2.0;
  unimplemented!()
}

pub fn bs_euro_gamma(cp:bool, k:f64, t:f64, s:f64, r:f64, g:f64, v:f64) -> f64 {
  let om = if cp {1.0} else {-1.0};
  let ert = (-r*t).exp();
  let egt = (-g*t).exp();
  let vqt = v * t.sqrt();
  let dp = ((s/k).ln() + (r-g)*t)/vqt + vqt/2.0;
  let dm = ((s/k).ln() + (r-g)*t)/vqt - vqt/2.0;
  unimplemented!()
}

pub fn bs_euro_vega(cp:bool, k:f64, t:f64, s:f64, r:f64, g:f64, v:f64) -> f64 {
  let om = if cp {1.0} else {-1.0};
  let ert = (-r*t).exp();
  let egt = (-g*t).exp();
  let vqt = v * t.sqrt();
  let dp = ((s/k).ln() + (r-g)*t)/vqt + vqt/2.0;
  let dm = ((s/k).ln() + (r-g)*t)/vqt - vqt/2.0;
  unimplemented!()
}

pub fn bs_euro_vanna(cp:bool, k:f64, t:f64, s:f64, r:f64, g:f64, v:f64) -> f64 {
  let om = if cp {1.0} else {-1.0};
  let ert = (-r*t).exp();
  let egt = (-g*t).exp();
  let vqt = v * t.sqrt();
  let dp = ((s/k).ln() + (r-g)*t)/vqt + vqt/2.0;
  let dm = ((s/k).ln() + (r-g)*t)/vqt - vqt/2.0;
  unimplemented!()
}

pub fn bs_euro_volga(cp:bool, k:f64, t:f64, s:f64, r:f64, g:f64, v:f64) -> f64 {
  let om = if cp {1.0} else {-1.0};
  let ert = (-r*t).exp();
  let egt = (-g*t).exp();
  let vqt = v * t.sqrt();
  let dp = ((s/k).ln() + (r-g)*t)/vqt + vqt/2.0;
  let dm = ((s/k).ln() + (r-g)*t)/vqt - vqt/2.0;
  unimplemented!()
}

pub fn bs_euro_rho(cp:bool, k:f64, t:f64, s:f64, r:f64, g:f64, v:f64) -> f64 {
  let om = if cp {1.0} else {-1.0};
  let ert = (-r*t).exp();
  let egt = (-g*t).exp();
  let vqt = v * t.sqrt();
  let dp = ((s/k).ln() + (r-g)*t)/vqt + vqt/2.0;
  let dm = ((s/k).ln() + (r-g)*t)/vqt - vqt/2.0;
  unimplemented!()
}

pub fn bs_euro_phi(cp:bool, k:f64, t:f64, s:f64, r:f64, g:f64, v:f64) -> f64 {
  let om = if cp {1.0} else {-1.0};
  let ert = (-r*t).exp();
  let egt = (-g*t).exp();
  let vqt = v * t.sqrt();
  let dp = ((s/k).ln() + (r-g)*t)/vqt + vqt/2.0;
  let dm = ((s/k).ln() + (r-g)*t)/vqt - vqt/2.0;
  unimplemented!()
}

pub fn bs_euro_theta(cp:bool, k:f64, t:f64, s:f64, r:f64, g:f64, v:f64) -> f64 {
  let om = if cp {1.0} else {-1.0};
  let ert = (-r*t).exp();
  let egt = (-g*t).exp();
  let vqt = v * t.sqrt();
  let dp = ((s/k).ln() + (r-g)*t)/vqt + vqt/2.0;
  let dm = ((s/k).ln() + (r-g)*t)/vqt - vqt/2.0;
  unimplemented!()
}

pub fn bs_euro_ddk(cp:bool, k:f64, t:f64, s:f64, r:f64, g:f64, v:f64) -> f64 {
  let om = if cp {1.0} else {-1.0};
  let ert = (-r*t).exp();
  let egt = (-g*t).exp();
  let vqt = v * t.sqrt();
  let dp = ((s/k).ln() + (r-g)*t)/vqt + vqt/2.0;
  let dm = ((s/k).ln() + (r-g)*t)/vqt - vqt/2.0;
  unimplemented!()
}