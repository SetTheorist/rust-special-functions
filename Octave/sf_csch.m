## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_csch (@var{z})
## Compute hyperbolic cosecant
## @end deftypefn

function res = sf_csch(z)
  res = 1 ./ sf_sinh(z);
endfunction
