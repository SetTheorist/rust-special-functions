## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_acoth (@var{z})
## Compute hyperbolic arc-cotangent
## @end deftypefn

function res = sf_acoth(z)
  res = -I*sf_acot(-I*z);
endfunction
