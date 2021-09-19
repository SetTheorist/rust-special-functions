## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_dawson (@var{z})
## Compute Dawson's integral $D(z) = e^(-z^2) \int_0^z e^(t^2) dt$ for real z
## correct only for reals
## @end deftypefn
function res = sf_dawson(z)
  if (nargin < 1) print_usage; endif
  res = zeros(size(z));
  for kk = 1:prod(size(z))
    res(kk) = sf_dawson__1(z(kk));
  endfor
endfunction

function res = sf_dawson__1(z)
  #dir = sf_exp(-z^2)*sf_erf(I*z)*sf_sqrt(pi)/2/I
  if (abs(z)<0.5)
    res = sf_exp(-z^2)*sf_erf(I*z)*sf_sqrt(pi)/2/I;
  elseif (!isreal(z))
    res = seres(z);
  elseif (abs(z)<5)
    res = contfrac(z);
  else
    res = contfrac2(z);
  endif
endfunction

function res = contfrac(x)
  eps = 1e-16;
  zeta = 1e-100;

  fj = 1;
  Cj = fj;
  Dj = 0;
  j = 1;
  do
    aj = (-1)^(rem(j,2)+1)*2*j*x^2;
    bj = 2*j+1;
    Dj = bj + aj*Dj; if (Dj==0) Dj=zeta; endif
    Cj = bj + aj/Cj; if (Cj==0) Cj=zeta; endif
    Dj = 1/Dj;
    Deltaj = Cj*Dj;
    fj *= Deltaj;
    ++j; if (j>999) break; endif
  until (abs(Deltaj-1)<eps)
  res = x/fj;
endfunction

function res = contfrac2(x)
  eps = 1e-16;
  zeta = 1e-100;

  fj = 1+2*x^2;
  Cj = fj;
  Dj = 0;
  j = 1;
  do
    aj = -4*j*x^2;
    bj = (2*j+1) + 2*x^2;
    Dj = bj + aj*Dj; if (Dj==0) Dj=zeta; endif
    Cj = bj + aj/Cj; if (Cj==0) Cj=zeta; endif
    Dj = 1/Dj;
    Deltaj = Cj*Dj;
    fj *= Deltaj;
    ++j; if (j>999) break; endif
  until (abs(Deltaj-1)<eps)
  res = x/fj;
endfunction

# from NR
# BUGGY
function res = rybicki(x)
  h = 2.0;
  n = 1;
  res = 0;
  do
    old_res = res;
    res += ( sf_exp(-(x-n*h)^2) - sf_exp(-(x+n*h)^2) )/n;
    n+=2; if (n>999) break; endif
  until (res == old_res)
  res /= sqrt(pi);
endfunction

function res = seres(x)
  res = term = x;
  n = 1;
  do
    term *= x^2 / n;
    old_res = res;
    res += term / (2*n+1);
    ++n; if (n>999) break; endif
  until (res == old_res)
  res *= sf_exp(-x^2);
endfunction

function res = besser2(x)
  res = 0;
  n = 1;
  do
    old_res = res;
    res += (2*n+1)*sf_bessel_spher_i1(n, x^2) + (2*n+3)*sf_bessel_spher_i1(n+1, x^2);
    n +=4 ; if (n>999) break; endif
  until (res == old_res)
  res *= sf_exp(-x^2) / x;
endfunction

function res = besser(x)
  res = 0;
  n = 0;
  do
    old_res = res;
    res += (-1)^(rem(n,2)) * (sf_bessel_spher_i1(2*n, x^2) + sf_bessel_spher_i1(2*n+1, x^2));
    ++n; if (n>999) break; endif
  until (res == old_res)
  res *= x * sf_exp(-x^2);
endfunction
