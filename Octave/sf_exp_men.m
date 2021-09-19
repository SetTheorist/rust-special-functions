## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_exp_men (@var{n}, @var{z})
## Compute tail of series expansion of exponential $e^z - \sum_(k=0)^(n-1) x^k/k!$
## ($n=0, 1, 2, ...$)
## @end deftypefn

function res = sf_exp_men(n, z)
  if (nargin<2 || !sf_is_nonnegint(n)) print_usage; endif
  res = ones(size(z));
  for k = 1:prod(size(z));
    if (isinf(z(k)))
      if (z(k)>0)
        res(k) = +Inf;
      else
        res(k) = -1;
      endif
    elseif (isnan(z(k)))
      res(k) = z(k);
    else
      res(k) = sf_exp_men_1(n, z(k));
    endif
  endfor
endfunction

function res = sf_exp_men_1(n, z)
  switch (n)
  case 0
    res = sf_exp(z);
  case 1
    res = sf_exp_m1(z);
  otherwise
    res = contfrac(n, z);
  endswitch
endfunction

# use Modified Lentz for continued fraction
function res = contfrac(n, z)
  zeta = 1e-150;
  eps = 1e-16;
  fj = n+1;
  Cj = fj;
  Dj = 0.0;
  j = 1;
  isodd = true;
  do
    if (isodd)
      aj = z*(j+1)/2;
    else
      aj = -z*(n+j/2);
    endif
    bj = n+1+j;
    Dj = bj + aj*Dj; if (Dj==0) Dj=zeta; endif
    Cj = bj + aj/Cj; if (Cj==0) Cj=zeta; endif
    Dj = 1/Dj;
    Deltaj = Cj*Dj;
    fj *= Deltaj;
    ++j;
    if (j>999); break; endif
    isodd = !isodd;
  until (abs(Deltaj - 1) < eps)
  res = z^n / (sf_factorial(n)*(1 - z/fj));
endfunction

