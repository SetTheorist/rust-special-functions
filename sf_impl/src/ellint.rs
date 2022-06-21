
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
  fn ellint_rg(self, y:Self, z:Self) -> Self;
  fn ellint_rj(self, y:Self, z:Self, p:Self) -> Self;
}

pub trait EllipticIntegralBurlisch : Value {
  fn ellint_cel(self, p:Self, a:Self, b:Self) -> Self;
  fn ellint_el1(self, x:Self) -> Self;
  fn ellint_el2(self, x:Self, a:Self, b:Self) -> Self;
  fn ellint_el3(self, x:Self, p:Self) -> Self;
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
#[inline]
pub fn sf_ellint_rd<V:EllipticIntegralSymmetric>(x:V, y:V, z:V) -> V { x.ellint_rd(y, z) }
#[inline]
pub fn sf_ellint_rf<V:EllipticIntegralSymmetric>(x:V, y:V, z:V) -> V { x.ellint_rf(y, z) }
#[inline]
pub fn sf_ellint_rg<V:EllipticIntegralSymmetric>(x:V, y:V, z:V) -> V { x.ellint_rg(y, z) }
#[inline]
pub fn sf_ellint_rj<V:EllipticIntegralSymmetric>(x:V, y:V, z:V, p:V) -> V { x.ellint_rj(y, z, p) }

#[inline]
pub fn sf_ellint_cel<V:EllipticIntegralBurlisch>(kc:V, p:V, a:V, b:V) -> V { kc.ellint_cel(p, a, b) }
#[inline]
pub fn sf_ellint_el1<V:EllipticIntegralBurlisch>(kc:V, x:V) -> V { kc.ellint_el1(x) }
#[inline]
pub fn sf_ellint_el2<V:EllipticIntegralBurlisch>(kc:V, x:V, a:V, b:V) -> V { kc.ellint_el2(x, a, b) }
#[inline]
pub fn sf_ellint_el3<V:EllipticIntegralBurlisch>(kc:V, x:V, p:V) -> V { kc.ellint_el3(x, p) }

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

impl EllipticIntegralThird for r64 {
  fn ellint_pi(self, c:Self) -> Self {
    impls::ell_pi(c, self)
  }
  fn ellint_pi_inc(self, c:Self, phi:Self) -> Self {
    // TODO: domain checking
    impls::ell_pi_incomplete(phi, c, self)
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
  fn ellint_rg(self, y:Self, z:Self) -> Self {
    impls::sym_rg_real(self, y, z)
  }
  fn ellint_rj(self, y:Self, z:Self, p:Self) -> Self {
    impls::sym_rj_real(self, y, z, p)
  }
}

impl EllipticIntegralBurlisch for r64 {
  fn ellint_cel(self, p:Self, a:Self, b:Self) -> Self {
    impls::burl_cel(self, p, a, b)
  }
  fn ellint_el1(self, x:Self) -> Self {
    impls::burl_el1(self, x)
  }
  fn ellint_el2(self, x:Self, a:Self, b:Self) -> Self {
    impls::burl_el2(self, x, a, b)
  }
  fn ellint_el3(self, x:Self, p:Self) -> Self {
    impls::burl_el3(self, x, p)
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

pub fn ell_pi<V:Value+AGM>(c:V, k:V) -> V {
  pi_complete_agm(c, k)
}

pub fn ell_pi_incomplete<V>(phi:V, c:V, k:V) -> V
  where V:Value+Log+Trig
    +EllipticIntegralFirst+EllipticIntegralSecond+EllipticIntegralSymmetric
{
  if phi == 0 {
    V::zero
  } else {
    pi_gauss_transform(phi, c, k)
  }
}

pub fn pi_complete_agm<V:Value+AGM>(c:V, k:V) -> V {
  let (an, gn, _) = sf_agm_vec(V::one, sf_kc(k), V::zero);
  let n = an.len();
  let mut pn = Vec::with_capacity(n);
  pn.push(sf_sqrt(ι(1):V - c));
  let mut qn = Vec::with_capacity(n);
  qn.push(ι(1):V);
  let mut en = Vec::with_capacity(n);
  en.push((pn[0].sqr() - an[0]*gn[0]) / (pn[0].sqr() + an[0]*gn[0]));
  for i in 1..n {
    pn.push((pn[i-1].sqr() + an[i-1]*gn[i-1]) / (pn[i-1]*2));
    en.push((pn[i].sqr() - an[i]*gn[i]) / (pn[i].sqr() + an[i]*gn[i]));
    qn.push(qn[i-1] * en[i-1]/2);
  }
  let mut qsum = V::zero;
  for &q in &qn { qsum += q; }
  V::PI/(an[n-1]*4) * (-c/(c-1)*qsum + 2)
}

// TODO: transform recursion to iteration
pub fn pi_gauss_transform<V>(phi:V, c:V, k:V) -> V
  where V:Value+Log+Trig
    +EllipticIntegralFirst+EllipticIntegralSecond+EllipticIntegralSymmetric
{
  let kp = sf_kc(k);
  // TODO: domain check

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
  let newgt = pi_gauss_transform(psi1, c1, k1);
  (newgt*4/(kp+1) + (rho-1)*sf_ellint_f(phi,k) - sf_ellint_rc(xi-1, xi-c))/rho
}

////////////////////////////////////////////////////////////////////////////////

pub fn burl_cel<V:Value+EllipticIntegralSymmetric>(kc:V, p:V, a:V, b:V) -> V {
  a * sf_ellint_rf(ι(0), kc.sqr(), ι(1)) + (b-p*a)/3 * sf_ellint_rj(ι(0), kc.sqr(), ι(1), p)
}

pub fn burl_el1<V:Value+EllipticIntegralSymmetric>(kc:V, x:V) -> V {
  let r = x.sqr().recip();
  sf_ellint_rf(r, r+kc.sqr(), r+1)
}
pub fn burl_el1_<V:Value+EllipticIntegralFirst+Trig>(kc:V, x:V) -> V {
   sf_ellint_f(sf_atan(x), sf_kc(kc))
}

pub fn burl_el2<V>(kc:V, x:V, a:V, b:V) -> V
  where V:Value+EllipticIntegralSymmetric
{
  let r = x.sqr().recip();
  a * burl_el1(kc, x) + (b-a)/3 * sf_ellint_rd(r, r+kc.sqr(), r+1)
}

pub fn burl_el3<V>(kc:V, x:V, p:V) -> V
  where V:Value+EllipticIntegralSymmetric+Trig
{
  let r = x.sqr().recip();
  burl_el1(kc, x) - (p-1)/3 * sf_ellint_rj(r, r+kc.sqr(), r+1, r+p)
}
pub fn burl_el3_<V>(kc:V, x:V, p:V) -> V
  where V:Value+EllipticIntegralThird+Trig
{
  sf_ellint_pi_inc(sf_atan(x), -p+1, sf_kc(kc))
}

////////////////////////////////////////////////////////////////////////////////

// for real parameters, x>=0, y!=0
//TODO:RealValue
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

////////////////////////////////////////

// for real x,y,z>0
//TODO:RealValue
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

////////////////////////////////////////
// for real x,y,z>0
//TODO:RealValue
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

////////////////////////////////////////
// for real x,y,z>0
pub fn sym_rg_real<V:RealValue+Log+Trig>(x:V, y:V, z:V) -> V {
  // TODO: domain check
  //let (x_,y_,z_) = (x,y,z)
  let (mut x, mut y, mut z) = (x, y, z);
  // sort x<=y<=z  (xy,xz,yz)
  if x>y {std::mem::swap(&mut x, &mut y);}
  if x>z {std::mem::swap(&mut x, &mut z);}
  if y>z {std::mem::swap(&mut y, &mut z);}
  let t0 = sf_sqrt(x);
  let mut tn = t0;
  let mut cn = sf_sqrt(y - x);
  let mut an = sf_sqrt(z - x);
  let h0 = sf_sqrt(z);
  let mut hn = h0;
  let theta = ι(1):V; //?
  let mut cn_sum = cn.sqr()/2;
  let mut hn_sum = V::zero;
  for n in 1..1000 {
    an = (an + sf_sqrt(an.sqr() - cn.sqr()))/2;
    tn = (tn + sf_sqrt(tn.sqr() + theta*cn.sqr()))/2;
    cn = cn.sqr()/(an*2)/2;
    cn_sum += cn.sqr()<<((n-1) as isize);
    let hnm1 = hn;
    hn = hn*tn/sf_sqrt(tn.sqr() + theta*cn.sqr());
    hn_sum += (hn - hnm1)<<(n as isize);
    if cn.sqr() == 0 {print!("<{}>",n);break;}
  }
  // TODO: warn if failure to converge
  ((t0.sqr()+theta*cn_sum)*sym_rc_real(tn.sqr()+theta*an.sqr(),tn.sqr()) + h0 + hn_sum)/2
}

////////////////////////////////////////
// for real x,y,z>0
pub fn sym_rj_real<V:RealValue+Log+Trig+Power>(x:V, y:V, z:V, p:V) -> V {
  // TODO: domain check
  let half23 : V = (ι(0.5):V).pow(ι(2):V/3); // 2^(-2/3) TODO: move to constants?  or trait constant?
  let mut scale = V::one;
  let mut sum = V::zero;
  let (mut x, mut y, mut z, mut p) = (x, y, z, p);
  for n in 0..1000 {
    let λ = sf_sqrt(x*y) + sf_sqrt(y*z) + sf_sqrt(z*x);
    let α = p * (sf_sqrt(x) + sf_sqrt(y) + sf_sqrt(z)) + sf_sqrt(x*y*z);
    let β = sf_sqrt(p) * (p + λ);
    let old = sum;
    if abs(α.sqr()/β.sqr() - 1) < V::epsilon*2 {
      // optimization to reduce calls
      sum += scale*3/α;
    } else {
      sum += scale*3*sym_rc_real(α.sqr(), β.sqr());
    }
    let mu = (x + y + z + p) / 4;
    let xyzp_old = (x,y,z,p);
    let eps = μ(x/mu-1).max(μ(y/mu-1)).max(μ(z/mu-1)).max(μ(p/mu-1));
    x = (x + λ)*half23/mu;
    y = (y + λ)*half23/mu;
    z = (z + λ)*half23/mu;
    p = (p + λ)*half23/mu;
    scale *= mu.pow(ι(-1.5):V);
    if eps<V::epsilon || xyzp_old==(x,y,z,p) || sum==old {print!("[{}]",n);break;}
  }
  // TODO: warn if failure to converge
  scale*x.pow(ι(-1.5):V) + sum
}

}
