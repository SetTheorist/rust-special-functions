## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_cosh_m1 (@var{z})
## Compute hyperbolic cosine minus 1, $\cosh(z)-1$
## @end deftypefn

function res = sf_cosh_m1(z)
  if (abs(z)>0.5)
    ee = sf_exp(z);
    res = (ee + 1./ee - 2)/2;
  else
    res = 0.0;
    z2 = z^2;
    n = 1;
    term = 1.0;
    do
      term *= z2 / ((2*n-1)*(2*n));
      old_res = res;
      res += term;
      ++n;
      if (n>999) break; endif
    until (res == old_res);
  endif
endfunction
