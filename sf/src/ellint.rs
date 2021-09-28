
pub trait EllipticIntegralFirst : Value {
  // complete elliptic integral of the first kind
  fn ellint_k(self) -> Self;
  // complete complementary elliptic integral of the first kind
  fn ellint_kc(self) -> Self { sf_kc(self).ellint_k() }
  // incomplete elliptic integral of the first kind
  fn ellint_f(self, phi:Self) -> Self;
}

pub trait EllipticIntegralSecond : Value {
  // complete elliptic integral of the second kind
  fn ellint_e(self) -> Self;
  // complete complementary elliptic integral of the second kind
  fn ellint_ec(self) -> Self { sf_kc(self).ellint_e() }
  // incomplete elliptic integral of the second kind
  fn ellint_e_inc(self, phi:Self) -> Self;
}

pub trait EllipticIntegralThird : Value {
  // complete elliptic integral of the third kind
  fn ellint_pi(self, c:Self) -> Self;
  // incomplete elliptic integral of the third kind
  fn ellint_pi_inc(self, c:Self, phi:Self) -> Self;
}

pub trait EllipticIntegralSymmetric : Value {
  fn ellint_rc(self, y:Self) -> Self;
  fn ellint_rd(self, y:Self, z:Self) -> Self;
  fn ellint_rf(self, y:Self, z:Self) -> Self;
}

#[inline]
pub fn sf_ellint_k<V:EllipticIntegralFirst>(k:V) -> V { k.ellint_k() }
#[inline]
pub fn sf_ellint_kc<V:EllipticIntegralFirst>(k:V) -> V { k.ellint_kc() }
#[inline]
pub fn sf_ellint_f<V:EllipticIntegralFirst>(phi:V, k:V) -> V { k.ellint_f(phi) }
#[inline]
pub fn sf_ellint_e<V:EllipticIntegralSecond>(k:V) -> V { k.ellint_e() }
#[inline]
pub fn sf_ellint_ec<V:EllipticIntegralSecond>(k:V) -> V { k.ellint_ec() }
#[inline]
pub fn sf_ellint_e_inc<V:EllipticIntegralSecond>(phi:V, k:V) -> V { k.ellint_e_inc(phi) }
#[inline]
pub fn sf_ellint_pi<V:EllipticIntegralThird>(c:V, k:V) -> V { k.ellint_pi(c) }
#[inline]
pub fn sf_ellint_pi_inc<V:EllipticIntegralThird>(phi:V, c:V, k:V) -> V { k.ellint_pi_inc(c, phi) }

#[inline]
pub fn sf_ellint_rc<V:EllipticIntegralSymmetric>(x:V, y:V) -> V { x.ellint_rc(y) }
pub fn sf_ellint_rd<V:EllipticIntegralSymmetric>(x:V, y:V, z:V) -> V { x.ellint_rd(y, z) }
pub fn sf_ellint_rf<V:EllipticIntegralSymmetric>(x:V, y:V, z:V) -> V { x.ellint_rf(y, z) }

#[inline]
pub fn sf_kc<V:Value>(k:V) -> V {
  sf_sqrt(V::one - k.sqr())
}

use crate::traits::*;
use crate::agm::*;
use crate::real::*;
use crate::trig::*;

impl EllipticIntegralFirst for r64 {
  fn ellint_k(self) -> Self {
    if (ι(1):r64 - self*self).is_negreal()  {
      ::log::warn!("Domain error EllipticIntegralFirst::<{}>::ellint_k({:e})", std::any::type_name::<Self>(), self);
      r64::nan
    } else {
      impls::ell_k(self)
    }
  }
  fn ellint_f(self, phi:Self) -> Self {
    if !phi.є(-r64::FRAC_PI_2, r64::FRAC_PI_2) {
      ::log::warn!("Domain error EllipticIntegralFirst::<{}>::ellint_f({:e})", std::any::type_name::<Self>(), self);
      r64::nan
    } else if (ι(1):r64 - (self*sf_sin(phi)).sqr()).is_negreal()  {
      ::log::warn!("Domain error EllipticIntegralFirst::<{}>::ellint_f({:e})", std::any::type_name::<Self>(), self);
      r64::nan
    } else {
      impls::ell_f(phi,self)
    }
  }
}

impl EllipticIntegralSecond for r64 {
  fn ellint_e(self) -> Self {
    impls::ell_e(self)
  }
  fn ellint_e_inc(self, phi:Self) -> Self {
    // TODO: domain checking
    impls::ell_e_incomplete(phi, self)
  }
}

impl EllipticIntegralSymmetric for r64 {
  fn ellint_rc(self, y:Self) -> Self {
    impls::sym_rc_real(self, y)
  }
  fn ellint_rd(self, y:Self, z:Self) -> Self {
    impls::sym_rd_real(self, y, z)
  }
  fn ellint_rf(self, y:Self, z:Self) -> Self {
    impls::sym_rf_real(self, y, z)
  }
}

pub mod impls {
use super::*;
use crate::agm::*;
use crate::log::*;
use crate::trig::*;
use crate::traits::*;


pub fn ell_k<V:Value+AGM>(k:V) -> V {
  let a = sf_agm(ι(1), sf_kc(k));
  V::PI / (a*2)
}

////////////////////////////////////////

pub fn ell_f<V:Value+Log+Trig>(phi:V, k:V) -> V {
  if k == 1 {
    sf_log((sf_sin(phi)+1)/(sf_sin(-phi)+1))/2
  } else if k == 0 {
    phi
  } else if phi == 0 {
    ι(0)
  } else {
    f_ascending_landen(phi, k)
  }
}

pub fn f_agm_method<V:Value+AGM>(phi:V, k:V) -> V {
  let (an,bn,cn,phin) = sf_agm_vec_extra(V::one, sf_kc(k), phi, k);
  let n = phin.len();
  phin[n-1] / (an[n-1]<<((n-1) as isize))
}

pub fn f_ascending_landen<V:Value+Log+Trig>(phi:V, k:V) -> V {
  let mut res = V::one;
  let mut k = k;
  let mut phi = phi;
  for n in 0..1000 {
    let k2 = sf_sqrt(k) * 2 / (k + 1);
    let phi2 = (phi + sf_asin(k * sf_sin(phi))) / 2;
    res *= ι(2):V / (k + 1);
    k = k2;
    phi = phi2;
    if k == ι(1):V {break;}
  }
  res *= sf_log((sf_sin(phi)+1) / (sf_sin(-phi)+1)) / 2;
  return res;
}

////////////////////////////////////////

pub fn ell_e<V:Value+AGM>(k:V) -> V {
  // TODO: domain
  let (va,_,vc) = sf_agm_vec(V::one, sf_kc(k), k);
  let n = vc.len();
  let mut res : V = -k.sqr() + 2;
  for i in 1..n {
    res -= vc[i].sqr() << (i as isize);
  }
  res * V::PI / (va[n-1]*4)
}

pub fn ell_e_incomplete<V:Value+Log+Trig>(phi:V, k:V) -> V {
  // TODO: domain
  if k == 1 {
    sf_sin(phi)
  } else if k == 0 {
    phi
  } else {
    e_ascending_landen(phi, k)
  }
}

// TODO: transform recursion into iteration
pub fn e_ascending_landen<V:Value+Log+Trig>(phi:V, k:V) -> V {
  if k == 1 {
    sf_sin(phi)
  } else {
    let k2 = sf_sqrt(k)*2/(k+1);
    let phi2 = (phi + sf_asin(k * sf_sin(phi)))/2;
    (k + 1)*e_ascending_landen(phi2, k2) + (-k + 1)*ell_f(phi2, k2) - k*sf_sin(phi)
  }
}

// TODO: buggy!
pub fn e_agm<V:Value+AGM+Trig>(phi:V, k:V) -> V {
  let (va,vb,vc,vphi) = sf_agm_vec_extra(V::one, sf_kc(k), k, phi);
  let n = vphi.len();
  let a = va[n-1];
  let ph = vphi[n-1];
  let mut cphi = V::zero;
  for i in 1..n { cphi += vc[i] * sf_sin(vphi[i]) };
  let mut c2 = V::zero;
  for i in 0..n { c2 += vc[i] << (i as isize); }
  ph / (a << ((n-1) as isize)) + cphi - (ph/(a<<((n+1) as isize))) * c2
}

////////////////////////////////////////

// TODO: transform recursion to iteration
pub fn gauss_transform<V>(phi:V, c:V, k:V) -> V
  where V:Value+Log+Trig
    +EllipticIntegralFirst+EllipticIntegralSecond+EllipticIntegralSymmetric
{
  let kp = sf_kc(k);

  if kp == 1 {
    let cp = sf_sqrt(ι(1):V - c);
    return sf_atan(cp * sf_tan(phi)) / cp;
  } else if V::one - (k.sqr()/c) == 0 {
    // special case else rho below is zero
    return (sf_ellint_e_inc(phi, k)
      - c*sf_cos(phi)*sf_sin(phi)/sf_sqrt(ι(1):V-c*sf_sin(phi).sqr()))/(-c+1);
  }

  let k1 = (-kp+1)/(kp+1);
  let delta = sf_sqrt(ι(1):V - k.sqr()*sf_sin(phi).sqr());
  let psi1 = sf_asin((kp+1)*sf_sin(phi)/(delta+1));
  let rho = sf_sqrt(V::one - (k.sqr()/c));
  let c1 = c*((rho+1)/(kp+1)).sqr();
  let xi = sf_csc(phi).sqr();
  let newgt = gauss_transform(psi1, c1, k1);
  (newgt*4/(kp+1) + (rho-1)*sf_ellint_f(phi,k) - sf_ellint_rc(xi-1, xi-c))/rho
}

////////////////////////////////////////

// for real parameters, x>=0, y!=0
pub fn sym_rc_real<V:Value+Log+Ordered+Trig>(x:V, y:V) -> V {
  if y == 0 || x.is_negreal() {
    ::log::warn!("Domain error impls::sym_rc_real::<{}>({},{})", std::any::type_name::<V>(), x, y);
    return V::nan;
  }

  if x == y {
    sf_sqrt_recip(x)
  } else if x.є(ι(0), y) {
    if x == 0 {
      sf_sqrt_recip(y-x) * sf_acos(sf_sqrt(x/y))
    } else {
      sf_sqrt_recip(y-x) * sf_atan(sf_sqrt((y-x)/x))
    }
  } else if y.є(ι(0),x) {
    sf_sqrt_recip(x-y) * sf_atanh(sf_sqrt((x-y)/x))
    //sf_sqrt(x-y))*sf_log((sf_sqrt(x)+sf_sqrt(x-y))/sf_sqrt(y))
  } else /*if y<ι(0):V && ι(0):V<=x*/ {
    sf_sqrt_recip(x-y) * sf_log((sf_sqrt(x)+sf_sqrt(x-y))/sf_sqrt(-y))
    //sf_sqrt_recip(x-y) * sf_atanh(sf_sqrt(x/(x-y)))
    //sf_sqrt(x/(x-y))*sym_rc_real(x-y, -y)
  }
}

// for real x,y,z>0
pub fn sym_rf_real<V:Value+Normed>(x:V, y:V, z:V) -> V {
  //let (x_, y_, z_) = (x, y, z);
  // TODO: domain check
  // sort x<y<z ?
  let (mut x,mut y,mut z) = (x,y,z);
  for n in 0..1000 {
    let λ = sf_sqrt(x*y) + sf_sqrt(y*z) + sf_sqrt(z*x);
    let mu = (x + y + z) / 3;
    let xyz_old = (x,y,z);
    let qx = x/mu-1;
    let qy = y/mu-1;
    let qz = z/mu-1;
    x = (x + λ)/4;
    y = (y + λ)/4;
    z = (z + λ)/4;
    let eps = qx.abs().max(qy.abs()).max(qz.abs());
    if eps<V::epsilon || xyz_old==(x,y,z) {break;}
  }
  // TODO: warn if failure to converge
  sf_sqrt_recip(x)
  // s2 = qqq.pow(2).sum()/4
  // s3 = (-qqq).pow(3).sum()/6
  // return sf_sqrt_recip(mu)*(1+s2/5+s3/7+s2*s2/6+s2*s3*3/11);
}

// for real x,y,z>0
pub fn sym_rd_real<V:Value+Power>(x:V, y:V, z:V) -> V {
  // TODO: domain check
  let half23 : V = (ι(0.5):V).pow(ι(2):V/3); // 2^(-2/3) TODO: move to constants?  or trait constant?
  //let half23 : V = ι(0.62996052494743658238);
  let mut sum : V = ι(0);
  //let (x_,y_,z_) = (x,y,z);
  let (mut x, mut y, mut z) = (x, y, z);
  for n in 0..1000 {
    let λ = sf_sqrt(x*y) + sf_sqrt(y*z) + sf_sqrt(z*x);
    sum += z.sqrt_recip()*3/(z+λ);
    let mu = (x + y + z)/3;
    let eps = μ(x/mu-1).max(μ(y/mu-1)).max(μ(z/mu-1));
    let xyz_old = (x,y,z);
    x = (x + λ) * half23;
    y = (y + λ) * half23;
    z = (z + λ) * half23;
    if eps < V::epsilon || xyz_old == (x, y, z) {
      break;
    }
  }
  // TODO: warn if failure to converge
  sum + x.pow(ι(-1.5):V)
}


}
