## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_cosh (@var{z})
## Compute hyperbolic cosine $\cosh(z)$
## @end deftypefn

function res = sf_cosh(z)
  res = ones(size(z));
  for kk = 1:prod(size(z))
    res(kk) = sf_cosh_1(z(kk));
  endfor
endfunction

function res = sf_cosh_1(z)
  if (abs(z)>0.25)
    ee = sf_exp(z);
    res = (ee + 1./ee)/2;
  else
    res = 1.0;
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
