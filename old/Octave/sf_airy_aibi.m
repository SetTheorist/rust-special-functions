## -*- texinfo -*-
## @deftypefn {Function File} {[@var{ai},@var{bi}] =} sf_airy_aibi (@var{z})
## Compute Airy functions Ai, Bi
## @end deftypefn

function [ai,bi] = sf_airy_aibi(z)
  if (nargin < 1)
    print_usage;
  endif
  # use bessel functions
  if (z<0)
    sqz3 = sqrt(-z)/3;
    zeta = (2/3) * (-z)^(3/2);
    jp = sf_bessel_j(1/3, zeta);
    jm = sf_bessel_j(-1/3, zeta);
    ai = (sqrt(-z)/3) * (jp + jm);
    if (nargout > 1)
      bi = sqrt(-z/3) * (jm - jp);
    endif
  elseif (z>0)
    sqz3 = sqrt(z/3);
    zeta = (2/3) * z^(3/2);
    ai = (sqz3/pi) * sf_bessel_k(1/3, zeta);
    if (nargout > 1)
      bi = sqz3 * (sf_bessel_i(1/3, zeta) + sf_bessel_i(-1/3, zeta));
    endif
  else
    persistent ai_0 = exp(-log(3)*2/3 - lgamma(2/3));
    persistent bi_0 = exp(-log(3)*1/6 - lgamma(2/3));
    ai = ai_0;
    bi = bi_0;
  endif
  ai_qua = sf_airy_quadrature(z)
  ai_ser = sf_airy_series_approach(z)
  ai_asy = sf_airy_asympt(z)
  ai_oct = airy(0, z)
  ai_bes = ai
endfunction

function [ai,bi] = sf_airy_quadrature(z)
  persistent cc = (sqrt(pi) * 48^(1/6) * sf_gamma(5/6));
  zeta = (2/3) * (z)^(3/2);
  az = (sf_exp(-zeta) * zeta^(-1/6)) / cc;
  int_r = quad(@(t)(real( (2+t/zeta)^(-1/6) * t^(-1/6) * sf_exp(-t) )), 0, Inf, 1e-14);
  int_i = quad(@(t)(imag( (2+t/zeta)^(-1/6) * t^(-1/6) * sf_exp(-t) )), 0, Inf, 1e-14);
  ai = az * (int_r + I*int_i);
endfunction

function [ai,bi] = sf_airy_asympt(z)
  zeta = (2/3)*z^(3/2);
  if (real(z)>0)
    # Ai
    res = term = mult = 1.0;
    k = 1;
    do
      mult /= -z*k*216;
      old_term = term;
      term = mult * prod((2*k+1):2:(6*k-1));
      if (abs(term) > abs(old_term)) break; endif
      old_res = res;
      res += term;
      ++k;
      if (k>999) break; endif
    until (res == old_res)
    k
    ai = res * sf_exp(-zeta) / (2*sqrt(pi*z^(1/2)));
    # Bi
    if (nargout > 1)
      res = term = mult = 1.0;
      k = 1;
      do
        mult /= z*k*216;
        old_term = term;
        term = mult * prod((2*k+1):2:(6*k-1));
        if (abs(term) > abs(old_term)) break; endif
        old_res = res;
        res += term;
        ++k;
        if (k>999) break; endif
      until (res == old_res)
      k
      bi = res * sf_exp(zeta) / (2*sqrt(pi*z^(1/2)));
    endif
  else
  endif
endfunction

function [ai,bi] = sf_airy_series_approach(z)
  persistent ai_0 = exp(-log(3)*2/3 - lgamma(2/3));
  persistent dai_0 = -exp(-log(3)*1/3 - lgamma(1/3));
  persistent bi_0 = exp(-log(3)*1/6 - lgamma(2/3));
  persistent dbi_0 = exp(log(3)*1/6 - lgamma(1/3));
  aibi1 = aibi_series_1(z);
  aibi2 = aibi_series_2(z);
  ai = ai_0 * aibi1 + dai_0 * aibi2;
  bi = bi_0 * aibi1 + dbi_0 * aibi2;
endfunction
function res = aibi_series_1(z)
  res = 1.0;
  term = 1.0;
  n = 1;
  z3 = z^3;
  do
    term *= z3 * (n*3-2) / ((n*3) * (n*3-1) * (n*3-2));
    old_res = res;
    res += term;
    ++n;
    if (n>999) break; endif
  until (res == old_res);
endfunction
function res = aibi_series_2(z)
  res = z;
  term = z;
  n = 1;
  z3 = z^3;
  do
    term *= z3 * (n*3-1) / ((n*3+1) * (n*3) * (n*3-1));
    old_res = res;
    res += term;
    ++n;
    if (n>999) break; endif
  until (res == old_res);
endfunction
