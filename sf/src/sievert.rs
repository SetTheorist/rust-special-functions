


pub trait Sievert {
  // $S(\theta, z) = \int_0^\theta \exp(-z * \sec(t))\,dt
  // for $0 \leq \theta \leq \pi/2$
  // (only for real z>=0 ?)
  fn sievert(self, theta:Self) -> Self;
}
pub fn sf_sievert<V:Sievert>(theta:V, z:V) -> V { z.sievert(theta) }


pub mod impls {
use crate::traits::{*};
use crate::exp::{Exp,sf_exp};
use crate::erf::{Erf,sf_erf};
use crate::trig::{Trig,sf_cos};


// special case when theta==pi/2 with special integral
pub fn sievert_pi2<V:Value>(z:V) -> V {
  // integrate sf_bessel_k(0, t) for t=z..Infinity
  // aka, according to Mathematica:
  // -(1/2) \[Pi] (-1 + z BesselK[0, z] StruveL[-1, z] + z BesselK[1, z] StruveL[0, z])
  unimplemented!("sievert_pi2({:?})", z)
}

// faster convergence for larger z/cos(theta)
// use for theta>pi/4 && z>1
// (otherwise just integrate the definition?!)
/*
// TODO: look at when we get sf_expint_en() implemented...
pub fn sievert_series<V:Value+Trig>(theta:V, z:V) -> V {
  let costh = sf_cos(theta);
  let mut res = sievert_pi2(z) - costh * sf_expint_en(z/costh, 2);
  let mut ak = costh * 1.0;
  for k in 1..1000 {
    ak *= costh.sqr() * (2*k-1) / (2*k);
    let old_res = res;
    res -= ak * sf_expint_en(z/costh, 2*k+2);
    if res == old_res { break; }
  }
  res
}
*/

use crate::integration::{*};
// just integrate the defining integral directly
pub fn sievert_integrate<V:Value+Exp+Trig>(theta:V, z:V) -> V {
  // TODO: use a more sensible integration method!
  Trapezoidal::new(ι(0),theta,100).integrate(|t:V|sf_exp(-z/sf_cos(t)))
}

// TODO: accuracy isn't great.
// formula validated (& matches A&S)
// (just expand sec at zero and take quadratic part & integrate)
// S(th,z) ~ exp(-z) * erf(th * \sqrt(z/2)) * \sqrt(pi/2z)
pub fn sievert_asympt<V:Value+Constants+Exp+Erf>(theta:V, z:V) -> V {
  sf_sqrt(V::PI/(z*2)) * sf_exp(-z) * sf_erf(theta * sf_sqrt(z/2))
}


}
