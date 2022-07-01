use crate::traits::*;

fn pythag<V:Value>(a:V, b:V) -> V {
  let a = a.abs();
  let b = b.abs();
  if a > b {
    ι(a):V * sf_sqrt((ι(b/a):V).sqr() + 1)
  } else if b == 0 {
    ι(0)
  } else {
    ι(b):V * sf_sqrt((ι(a/b):V).sqr() + 1)
  }
}

// computes eigenvalues of real, symmetric, tri-diagonal matrix
// using "QL" algorithm (based on Numerical Recipes)
// on input, d contains diagonal and e contains sub-diagonal elements (e[0] is not used)
// on output, d contains eigenvalues and e contains garbage
pub fn eig_symtrid<V:RealValue+Float>(d:&mut [V], e:&mut [V]) {
  let n = d.len();
  // reindex e for convenience
  for i in 1..n { e[i-1] = e[i]; }
  e[n-1] = ι(0);
  for l in 0..n {
    let mut iter = 0;
    loop {
      // look for single small subdiagonal element to split the matrix
      let mut m = n-1;
      for mm in l..(n-1) {
        let dd = d[mm].abs() + d[mm+1].abs();
        if e[mm].abs() <= V::epsilon * dd { m = mm; break; }
      }
      if m != l {
        iter += 1;
        if iter == 30 { log::warn!("Too many iterations"); break; }
        let mut g = (d[l+1] - d[l]) / (e[l] * 2);
        let mut r = pythag(g, ι(1));
        g = d[m] - d[l] + e[l] / (g + r.copysign(g));
        let mut s = ι(1);
        let mut c = ι(1);
        let mut p = ι(0);
        let mut i_flag = false;
        for ii in (l..m).rev() {
          let f = s * e[ii];
          let b = c * e[ii];
          r = pythag(f, g);
          e[ii+1] = r;
          if r == 0 {
            d[l+1] -= p;
            e[m] = ι(0);
            i_flag = true;
            break;
          }
          s = f / r;
          c = g / r;
          g = d[ii+1] - p;
          r = (d[ii] - g)*s + c*b*2;
          p = s * r;
          d[ii+1] = g + p;
          g = c*r - b;
        }
        if r == 0 && i_flag { continue; }
        d[l] -= p;
        e[l] = g;
        e[m] = ι(0);
      }
      if m == l { break; }
    }
  }
}