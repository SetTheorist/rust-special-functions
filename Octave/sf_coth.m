## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_coth (@var{z})
## Compute hyperbolic cotangent
## @end deftypefn

function res = sf_coth(z)
  res = 1 ./ sf_tanh(z);
endfunction
