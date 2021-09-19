## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_asec (@var{z})
## Compute arc-secant
## @end deftypefn

function res = sf_asec(z)
  res = sf_acos(1./z);
endfunction
