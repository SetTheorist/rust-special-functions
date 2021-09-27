## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_abramowitz (@var{m}, @var{z})
## Compute integral $f_m(x)=\int_0^\infty t^m e^(-t^2 - x/t) dx$ from Abramowitz & Stegun
## @end deftypefn
function res = sf_abramowitz(m, z)
  if (nargin < 2)
    print_usage;
  endif
  if (m!=fix(m) || m<0 || imag(m)!=0) res = NaN; return; endif
  if (m==1 && abs(z)<10)
    # power series for case m==1
    res = sf_abramowitz_series_m1(z);
  else
    # quadrature
    if (imag(z)==0)
      res = quad(@(t)(t^m * sf_exp(-t^2-z/t)), 0, Inf);
    else
      res = quad(@(t)(real(t^m * sf_exp(-t^2-z/t))), 0, Inf) + I*quad(@(t)(imag(t^m * sf_exp(-t^2-z/t))), 0, Inf);
    endif
  endif
endfunction

function res = sf_abramowitz_series_m1(z)
  lnz = sf_log(z);
  ak1 = 0.0;
  bk1 = -sqrt(pi);
  ak  = -1;
  bk  = (3/2)*(1 - 0.57721566490153286061);
  res = 1 + (ak1*lnz + bk1)*z + (ak*lnz + bk)*z^2;
  k = 3;
  do
    ak2 = ak1;
    bk2 = bk1;
    ak1 = ak;
    bk1 = bk;
    ak = (-2*ak2) / (k*(k-1)*(k-2));
    bk = (-2*bk2 - (3*k^2 - 6*k + 2)*ak) / (k*(k-1)*(k-2));
    old_res = res;
    res += (ak*lnz + bk)*z^k;
    ++k;
    if (k>999) break; endif
  until (res == old_res)
  res *= 0.5;
endfunction
