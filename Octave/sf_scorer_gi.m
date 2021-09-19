## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_scorer_gi (@var{z})
## Compute the Scorer function (inhomogeneous Airy function) Gi
## series for now (only small |z|)
## @end deftypefn

function res = sf_scorer_gi(z)
  if (nargin < 1) print_usage; endif
  res = ones(size(z));
  for kk = 1:prod(size(z))
    if (abs(z(kk))<2 || (!isreal(z(kk))&&abs(z(kk)<5)))
      res(kk) = gi_series(z(kk));
    elseif (abs(z(kk))<10 && isreal(z(kk)))
      res(kk) = gi_quad(z(kk));
    else
      res(kk) = gi_asympt(z(kk));
    endif
  endfor
endfunction

function res = gi_quad(z)
  persistent sq3 = sqrt(3)/2;
  persistent pi23 = pi*2/3;
  res = -quad(@(t)( sf_exp(-t^3/3 - z*t/2)*sf_cos(z*t*sq3 + pi23) ), 0, inf, 1e-16)/pi;
endfunction

function res = gi_series(z)
  res = 0.0;
  k = 0;
  do
    old_res = res;
    res += -z^k * 3^((k-2)/3) * sf_gamma((k+1)/3) * sf_cos((k+1)*pi*2/3) / sf_factorial(k);
    ++k;
    if (k>999) break; endif
  until (res == old_res)
  res /= pi;
endfunction

function res = gi_asympt(z)
  term = res = 1.0;
  k = 1;
  z3 = 3*z^3;
  do
    old_term = term;
    term *= (3*k)*(3*k-1)*(3*k-2) / k / z3;
    if (abs(term) > abs(old_term)) break; endif
    old_res = res;
    res += term;
    ++k;
    if (k>999) break; endif
  until (res == old_res)
  res /= pi*z;
endfunction
