## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_asinh (@var{z})
## Compute hyperbolic arc-sine
## @end deftypefn

function res = sf_asinh(z)
  res = I*sf_asin(-I*z);
endfunction
