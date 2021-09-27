## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_acosh (@var{z})
## Compute hyperbolic arc-cosine
## @end deftypefn

function res = sf_acosh(z)
  res = I*sf_acos(z);
endfunction
