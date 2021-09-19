## -*- texinfo -*-
## @deftypefn {Function File} {[@var{ai},@var{bi}] =} sf_airy (@var{z})
## Compute Airy functions Ai, Bi
## @end deftypefn

function [ai,bi] = sf_airy(z)
  if (nargin < 1) print_usage; endif
  # use bessel functions
  if (z<0)
    sqz3 = sf_sqrt(-z)/3;
    zeta = (2/3) * (-z)^(3/2);
    jp = sf_bessel_j(zeta, 1/3);
    jm = sf_bessel_j(zeta, -1/3);
    ai = (sf_sqrt(-z)/3) * (jp + jm);
    if (nargout > 1)
      bi = sf_sqrt(-z/3) * (jm - jp);
    endif
  elseif (z>0)
    sqz3 = sf_sqrt(z/3);
    zeta = (2/3) * z^(3/2);
    ai = (sqz3/pi) * sf_bessel_k(zeta, 1/3);
    if (nargout > 1)
      bi = sqz3 * (sf_bessel_i(zeta, 1/3) + sf_bessel_i(zeta, -1/3));
    endif
  else
    persistent ai_0 = sf_exp(-sf_log(3)*2/3 - sf_ln_gamma(2/3));
    persistent bi_0 = sf_exp(-sf_log(3)*1/6 - sf_ln_gamma(2/3));
    ai = ai_0;
    bi = bi_0;
  endif
endfunction

function [ai,bi] = sf_airy_series_approach(z)
  persistent ai_0 = sf_exp(-sf_log(3)*2/3 - sf_ln_gamma(2/3));
  persistent dai_0 = -sf_exp(-sf_log(3)*1/3 - sf_ln_gamma(1/3));
  persistent bi_0 = sf_exp(-sf_log(3)*1/6 - sf_ln_gamma(2/3));
  persistent dbi_0 = sf_exp(sf_log(3)*1/6 - sf_ln_gamma(1/3));
  aibi1 = aibi_1(z);
  aibi2 = aibi_2(z);
  ai = ai_0 * aibi1 + dai_0 * aibi2;
  bi = bi_0 * aibi1 + dbi_0 * aibi2;
endfunction

function res = aibi_1(z)
  res = 1.0;
  term = 1.0;
  n = 1;
  z3 = z^3;
  do
    term *= z3 * (n*3-2) / ((n*3) * (n*3-1) * (n*3-2));
    old_res = res;
    res += term;
    ++n; if (n>999) break; endif
  until (res == old_res);
endfunction

function res = aibi_2(z)
  res = z;
  term = z;
  n = 1;
  z3 = z^3;
  do
    term *= z3 * (n*3-1) / ((n*3+1) * (n*3) * (n*3-1));
    old_res = res;
    res += term;
    ++n; if (n>999) break; endif
  until (res == old_res);
endfunction
