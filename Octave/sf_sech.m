## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_sech (@var{z})
## Compute hyperbolic secant
## @end deftypefn

function res = sf_sech(z)
  res = 1 ./ sf_cosh(z);
endfunction
