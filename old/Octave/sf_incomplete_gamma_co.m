## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_incomplete_gamma_co (@var{a}, @var{x})
## Compute upper incomplete Gamma function -- $\Gamma(a,x) = \int_x^\infty ...$
## -- wrapper for now
## @end deftypefn

function res = sf_incomplete_gamma_co(a, x)
  if (nargin<2) print_usage; endif
  res = gammainc(x, a, "upper")*sf_gamma(a)
  res = contfrac(a,x)
endfunction

# works best for a>>z
function res = contfrac(a, z)
  persistent eps = 1e-17;
  persistent zeta = 1e-100;
  fj = z;
  Cj = fj;
  Dj = 0;
  j = 1;
  do
    if (rem(j,2)==0)
      aj = j/2;
      bj = z;
    else
      aj = (j+1)/2 - a;
      bj = 1;
    endif
    Dj = bj + aj*Dj; if (Dj==0) Dj=zeta; endif
    Cj = bj + aj/Cj; if (Cj==0) Cj=zeta; endif
    Dj = 1/Dj;
    Deltaj = Cj*Dj;
    fj *= Deltaj;
    ++j; if (j>999) break; endif
  until (abs(Deltaj - 1)<eps)
  if (j>999)
    warning('sf:convergence',
        "sf_incomplete_gamma_co(%g,%g) - continued-fraction failed to converge after %i iterations", a, z, j);
  endif
  if (abs(z)>100 || abs(a)>100) # simple-minded check
    res = sf_exp(a*sf_log(z) - z - sf_log(fj));
  else
    res = z^a * sf_exp(-z) * 1/fj;
  endif
endfunction
