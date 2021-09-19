## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_atanh (@var{z})
## Compute hyperbolic arc-tangent
## @end deftypefn

function res = sf_atanh(z)
  res = I*sf_atan(-I*z);
endfunction
