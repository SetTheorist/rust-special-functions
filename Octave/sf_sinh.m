## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_sinh (@var{z})
## Compute hyperbolic sine $\sinh(z)$
## @end deftypefn

function res = sf_sinh(z)
  res = (sf_exp_m1(z) - sf_exp_m1(-z))/2;
endfunction
