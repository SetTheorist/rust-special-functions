## -*- texinfo -*-
## @deftypefn {Function File} {[@var{gi},@var{hi}] =} sf_scorer_gihi (@var{z})
## Compute the Scorer functions (inhomogeneous Airy functions) Gi, Hi
## series for now (only small |z|)
## @end deftypefn

function [gi,hi] = sf_scorer_gihi(z)
  if (nargin < 1)
    print_usage;
  endif
  gi_ser = gi_series(z)
  hi_ser = hi_series(z)
  gi = gi_ser;
  hi = hi_ser;
endfunction

function res = gi_series(z)
  res = 0.0;
  k = 0;
  do
    old_res = res;
    res += -z^k * 3^((k-2)/3) * sf_gamma((k+1)/3) * sf_cos((k+1)*pi*2/3) / sf_factorial(k);
    ++k;
    if (k>999) break; endif
  until (res == old_res);
  k
  res /= pi;
endfunction
function res = hi_series(z)
  res = 0.0;
  k = 0;
  do
    old_res = res;
    res += z^k * 3^((k-2)/3) * sf_gamma((k+1)/3) / sf_factorial(k);
    ++k;
    if (k>999) break; endif
  until (res == old_res);
  k
  res /= pi;
endfunction
