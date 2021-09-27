## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_tan (@var{z})
## Compute tangent $\tan(z) = \sin(z)/\cos(z)$
## @end deftypefn

function res = sf_tan(z)
  res = sf_sin(z) ./ sf_cos(z);
  # often faster but less accurate:
  #res = contfrac(z)
endfunction

function res = contfrac(z)
  zeta = 1e-300;
  eps = 1e-16;
  fj = 1.0;
  Cj = fj;
  Dj = 0.0;
  j = 1;
  aj = -z^2;
  while (true)
    bj = 2*j+1;
    Dj = bj + aj*Dj; if (Dj==0) Dj=zeta; endif
    Cj = bj + aj/Cj; if (Cj==0) Cj=zeta; endif
    Dj = 1/Dj;
    Delta = Cj*Dj;
    fj *= Delta;
    if (abs(Delta-1)<eps) break; endif
    ++j; if (j>999) break; endif
  endwhile
  res = z/fj;
endfunction
