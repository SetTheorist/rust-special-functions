## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_digamma (@var{z})
## Compute digamma function $\psi(z) = \frac{\Gamma'(z)}{\Gamma(z)}$
## @end deftypefn

function res = sf_digamma(z)
  if (nargin < 1) print_usage; endif
  res = zeros(size(z));
  for n = 1:prod(size(z))
    res(n) = sf_digamma_1(z(n));
  endfor
endfunction

function res = sf_digamma_1(z)
  if (sf_is_nonposint(z)) res = Inf; return; endif
  if (abs(z)>10)
    res = sf_digamma_1_asympt(z);
  else
    res = sf_digamma_1_series(z);
  endif
endfunction

# series with Euler-Maclaurin correction
function res = sf_digamma_1_series(z)
  res = sm = -0.5772156649015328606065120901 - 1/z;
  k = 1;
  persistent bn = sf_bernoulli_number_scaled([2,4,6,8]) .* [1,6,120,5040];
  do
    trm = z / (k * (k+z));
    sm += trm;
    old_res = res;
    res = sm + sf_log((k+z)/k) - trm/2 ...
        + bn(1)*(k^(-2) - (k+z)^(-2)) ...
        + bn(2)*(k^(-4) - (k+z)^(-4)) ...
        + bn(3)*(k^(-6) - (k+z)^(-6)) ...
        + bn(4)*(k^(-8) - (k+z)^(-8)) ...
        ;
    ++k; if (k>999) break; endif
  until (res == old_res)
endfunction

# asymptotic expansion for |arg z|<pi
function res = sf_digamma_1_asympt(z)
  if (z<0.5)
    z = 1-z;
    term = res = -pi/sf_tan(pi*(1-z)) + sf_log(z) - 1/(2*z);
  else
    term = res = sf_log(z) - 1/(2*z);
  endif
  m = 0;
  z2m = 1.0;
  z_2 = z^(-2);
  do
    z2m *= z_2;
    old_term = term;
    # TODO: cache
    term = z2m * sf_bernoulli_number(2*m+2) / (2*m+2);
    old_res = res;
    res -= term;
    ++m; if (m>999) break; endif
  until (res == old_res) || (abs(term) > abs(old_term))
endfunction

