## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_vsin (@var{z})
## Compute the versine function
## @end deftypefn

function res = sf_vsin(z)
  res = 2 * sf_sin(z / 2) .^ 2;
endfunction
