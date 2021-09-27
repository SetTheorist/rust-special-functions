## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_polylog (@var{s}, @var{z})
## Compute the polylogarithm $Li_s(z) = \sum_(n=1)^\infty z^n / n^s$
## @end deftypefn

function res = sf_polylog(s, z)
  if (nargin < 2) print_usage; endif
  if (abs(z)<.75)
    res = series(s,z);
  elseif (imag(s)!=0 || s!=fix(s))
    res = zeta_series(s,z);
  elseif ( (real(s)>0 && abs(arg(1-z))<pi) || (real(s)>1 && z==1) )
    # unfortunately, quad() will fail for complex arguments :-/
    res = (z/sf_gamma(s)) * ( ...
              quad(@(t)(real(t^(s-1) / (exp(t)-z))), 0, Inf, 1e-12) ...
              +I* quad(@(t)(imag(t^(s-1) / (exp(t)-z))), 0, Inf, 1e-12));
  else
    res = nan;
  endif
endfunction

# valid for any s with |z|<1
# more accurate than zeta series below, but slower converging for
# small s or |z|~1...
function res = series(s, z)
  res = 0.0;
  zn = 1.0;
  n = 1;
  e_ = 0;
  do
    zn *= z;
    old_res = res;
    #res += zn / n^s;
    t_ = res;
    y_ = (zn / n^s) + e_;
    res = t_ + y_;
    e_ = (t_ - res) + y_;
    ++n;
    if (n>999) break; endif
  until (res == old_res)
  res += e_;
endfunction

# zeta series valid only for |ln(z)<2\pi| and s!=1,2,3,...
# (e^(2pi)=535.49 but the series fails before then)
# accuracy drops for s ~ integers
function res = zeta_series(s, z)
  lnz = sf_log(z);
  res = sf_gamma(1 - s) * (-lnz)^(s-1) + sf_zeta(s);
  n = 1;
  term = 1.0;
  e_ = 0;
  do
    term *= lnz / n;
    old_res = res;
    #res += sf_zeta(s-n) * term;
    t_ = res;
    y_ = (sf_zeta(s-n) * term) + e_;
    res = t_ + y_;
    e_ = (t_ - res) + y_;
    ++n;
    if (n>999) break; endif
  until (res == old_res)
  res += e_;
endfunction
