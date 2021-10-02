use crate::traits::*;
use crate::trig::*;

// a+bx=0
pub fn solve_linear<V:Value>(a:V, b:V) -> V {
  -a/b
}

// a+bx+cx^2=0
// TODO: more cleanly deal with real/complex cases...
// TODO: do this in an actually robust manner!
pub fn solve_quadratic<V:Value>(a:V, b:V, c:V) -> (V,V) {
  let (r1, r2);
  if c == 0 {
    r1 = solve_linear(a, b);
    r2 = V::nan;
  } else {
    let δ = sf_sqrt(b.sqr() - a*c*4);
    r1 = (-b + δ)/(c*2);
    r2 = (-b - δ)/(c*2);
  }
  (r1, r2)
}

// a+bx+cx^2+dx^3=0
// TODO: check for repeated roots, make robust, etc.
// TODO: deal more cleanly with real / complex cases
pub fn solve_cubic<V:Value+Trig>(a:V, b:V, c:V, d:V) -> (V,V,V) {
  let (r1, r2, r3);
  if d == 0 {
    (r1, r2) = solve_quadratic(a, b, c);
    r3 = V::nan;
  } else {
    // get equivalent "depressed" cubic: t^3+pt+q=0
    let p = (b*d*3 - c.sqr())/(d.sqr()*3);
    let q = (c.cub()*2 - b*c*d*9 + a*d.sqr()*27)/(d.cub()*27);
    // trigonometric approach
    let t = sf_sqrt(-p*4/3);
    let α = sf_acos(-q*4/t.cub());
    let c3d = -c/(d*3);
    r1 = c3d + t*sf_cos(α/3);
    r2 = c3d + t*sf_cos(α/3 + V::PI*2/3);
    r3 = c3d + t*sf_cos(α/3 + V::PI*4/3);
  }
  // TODO: maybe a step of Newton to "polish"?
  //let polish = |x|{x - (((x*d+c)*x+b)*x+a)/((x*d*3+c*2)*x+b)};
  //let (r1, r2, r3) = (polish(r1), polish(r2), polish(r3));
  (r1, r2, r3)
}

// a+bx+cx^2+dx^3+ex^4=0
// TODO: make robust, check for repeated roots, etc.
// TODO: deal more cleanly with real/complex cases
// uses approach from wikipedia "quartic function" article
pub fn solve_quartic<V:Value+Trig>(a:V, b:V, c:V, d:V, e:V) -> (V,V,V,V) {
  if e == 0 {
    let (r1,r2,r3) = solve_cubic(a, b, c, d);
    let r4 = V::nan;
    (r1,r2,r3,r4)
  } else {
    // normalize
    let a = a/e;
    let b = b/e;
    let c = c/e;
    let d = d/e;
    let e = V::one;
    // transform to "depressed" quartic
    let cc = c - d.sqr()*3/8;
    let dd = d.cub()/8 - d*c/2 + b;
    let ee = a - d.pow(4)*3/256 + d.sqr()*c/16 - b*d/4;
    // solve resolvent cubic
    // P^3 + 2cP^2 + (c^2-4e)P - d^2 = 0
    // where P=p^2
    let (p1,p2,p3) = solve_cubic(-dd.sqr(), cc.sqr()-ee*4, cc*2, V::one);
    let p = sf_sqrt(p3);
    let r = -p;
    let s = (cc + p.sqr() + dd/p)/2;
    let q = (cc + p.sqr() - dd/p)/2;
    // thus x^4+bx^3+cx^2+dx+e = (x^2+px+q)(x^2+rx+s)
    let (r1,r2) = solve_quadratic(q, p, V::one);
    let (r3,r4) = solve_quadratic(s, r, V::one);
    let r1 = r1 - d/4;
    let r2 = r2 - d/4;
    let r3 = r3 - d/4;
    let r4 = r4 - d/4;
    (r1, r2, r3, r4)
  }
}
