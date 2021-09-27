## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_cosint (@var{z})
## Compute the cosine-integral $Ci(z)$
## @end deftypefn

function res = sf_cosint(z)
  if (nargin<1) print_usage; endif
  res = zeros(size(z));
  for kk = 1:prod(size(z))
    if (abs(z(kk))<5)
      res(kk) = series(z(kk));
    else
      if (isreal(z(kk)))
        res(kk) = -real(e1_cf(I*z(kk)));
        if (z(kk)<0) res(kk) += pi*I; endif
      else
        res(kk) = -(e1_cf(I*z(kk)) + e1_cf(-I*z(kk)))/2;
      endif
    endif
    #res(kk) = asympt(z(kk));
  endfor
endfunction

function res = series(z)
  # TODO: initialize e_ with tail of constant...
  e_ = 0;
  res = 0.57721566490153286061;
  #res += sf_log(z);
      t_ = res;
      y_ = (sf_log(z)) + e_;
      res = t_ + y_;
      e_ = (t_ - res) + y_;
  term = 1.0;
  n = 1;
  z2 = -z^2;
  do
    term *= z2 / ((2*n-1)*(2*n));
    old_res = res;
    #res += term/(2*n);
      t_ = res;
      y_ = (term/(2*n)) + e_;
      res = t_ + y_;
      e_ = (t_ - res) + y_;
    ++n;
    if (n>999) break; endif
  until (res == old_res);
endfunction

# continued fraction for E_1(z)
function res = e1_cf(z)
  eps = 1e-16;
  zeta = 1e-150;
  fj = z; if (fj==0) fz=zeta; endif
  Cj = fj;
  Dj = 0;
  n = 1;
  do
    aj = fix((n+1)/2);
    if (rem(n,2)==0)
      bj = z;
    else
      bj = 1;
    endif
    Dj = bj + aj*Dj; if (Dj==0) Dj=zeta; endif
    Cj = bj + aj/Cj; if (Cj==0) Cj=zeta; endif
    Dj = 1/Dj;
    Deltaj = Cj*Dj;
    fj *= Deltaj;
    ++n; if (n>999) break; endif
  until (abs(Deltaj - 1)<eps)
  res = sf_exp(-z)/fj;
endfunction

function res = asympt(z)
  if (real(z)<0) z = -z; endif
  res = sf_sin(z)*asympt_f(z) - sf_cos(z)*asympt_g(z);
endfunction

function res = asympt_f(z)
  res = term = 1.0;
  e_ = 0;
  n = 1;
  z2 = z^2;
  do
    old_term = term;
    term *= -(2*n)*(2*n-1)/z2;
    if (abs(term)>abs(old_term)) break; endif
    old_res = res;
    #res += term;
      t_ = res;
      y_ = (term) + e_;
      res = t_ + y_;
      e_ = (t_ - res) + y_;
    ++n; if (n>999) break; endif
  until (res == old_res)
  res /= z;
endfunction

function res = asympt_g(z)
  res = term = 1.0;
  e_ = 0;
  n = 1;
  z2 = z^2;
  do
    old_term = term;
    term *= -(2*n)*(2*n+1)/z2;
    if (abs(term)>abs(old_term)) break; endif
    old_res = res;
    #res += term;
      t_ = res;
      y_ = (term) + e_;
      res = t_ + y_;
      e_ = (t_ - res) + y_;
    ++n; if (n>999) break; endif
  until (res == old_res)
  res /= z2;
endfunction

