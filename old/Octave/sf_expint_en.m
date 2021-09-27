## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_expint_en (@var{z}, @var{n})
## Compute exponential integral $\O{E}_n(z)$
## @end deftypefn

function res = sf_expint_en(z,n)
  if (nargin < 2) print_usage; endif
  res = zeros(size(z));
  for kk = 1:prod(size(z))
    res(kk) = en__1(z(kk),n);
  endfor
endfunction
function res = en__1(z,n)
  if (n<0) res = nan; return; endif
  if (n==0)
    res = sf_expint_en_0(z);
  elseif (n==1)
    res = sf_expint_en_1(z);
  else
    if (z<=1.0)
      res = sf_expint_en_series(z,n);
    else
      res = sf_expint_en_contfrac(z,n);
    endif
  endif
endfunction

function res = sf_expint_en_0(z)
  res = sf_exp(-z)/z;
endfunction

function res = sf_expint_en_1(z)
  res = -0.57721566490153286061 - sf_log(z);
  term = -1.0;
  k = 1;
  do
    term *= -1 * z / k;
    old_res = res;
    res += term / k;
    ++k;
    if (k>999) break; endif
  until (res == old_res);
endfunction

# assume n>=2, x<=1
function res = sf_expint_en_series(z,n)
  res = (-log(z) + sf_digamma(n)) * (-z)^(n-1)/factorial(n-1) + 1.0/(n-1);
  m = 1;
  term = 1.0;
  old_res = 0.0;
  do
    term *= -z/m;
    if (m==n-1)
      ++m;
      continue;
    endif
    old_res = res;
    res -= term / (m - (n-1));
    ++m;
    if (m>999) break; endif
  until (res == old_res);
endfunction

# assume n>=2, z>1
function res = sf_expint_en_contfrac(z,n)
  zeta = 1e-100;
  eps = 5e-16;
  # modified Lentz algorithm
  fj = zeta;
  Cj = fj;
  Dj = 0;
  j = 1;
  do
    if (j==1) aj=1; else aj=-(j-1)*(n+j-2); endif
    bj = z + n + 2*(j-1);
    Dj = bj + aj*Dj; if (Dj==0.0) Dj=zeta; endif
    Cj = bj + aj/Cj; if (Cj==0.0) Cj=zeta; endif
    Dj = 1/Dj;
    Delta = Cj*Dj;
    fj = fj*Delta;
    ++j;
    if (j>999) break; endif
  until (abs(Delta-1)<eps)
  res = fj * sf_exp(-z);
endfunction



