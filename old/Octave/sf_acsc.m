## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_acsc (@var{z})
## Compute arc-cosecant
## @end deftypefn

function res = sf_acsc(z)
  res = sf_asin(1./z);
endfunction
