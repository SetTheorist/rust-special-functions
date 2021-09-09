use crate::real::*;


pub fn bessel_j_series(z:r64, nu:r64) -> r64 {
  let z2 = -(z/2)*(z/2);
  let terms = (1..).scan(r64(1.0), {|s,m|{*s *= z2/m/(m+nu);Some(*s)});
  let terms = std::iter::once(ι(1)).chain(terms);
  //sumit(terms, 1e-16) * (z/2)^nu / gamma(nu+1)
  unimplemented!()
}

// for |z|>>nu, |arg z|<pi
pub fn bessel_j_asymp_z(z:r64, nu:r64) -> r64 {
  //let chi = z - (nu/2 + 0.25)*PI;
  //let mu = 4 * (nu^2);
  // sqrt(2/(pi*z)) * (asymp_p(z,nu)*sf_cos(chi) - asymp_q(z,nu)*sin(chi))
  unimplemented!()
}
/*
fn asymp_p(z:r64, nu:r64) -> r64 {
  let mu = 4*(nu^2);
  let mut res = ι(1);
  let mut term = ι(1);
  let z8 = -(8*z)^2;
  for k in 1..1000 {
    let old_term = term;
    term *= (mu - (2*k-1).sqr()) * (mu - (2*k+1).sqr()^2) / ((2*k-1)*(2*k)*z8);
    let old_res = res;
    res += term;
    if res == old_res || term.abs() > old_term.abs() { break; }
  }
}
*/


