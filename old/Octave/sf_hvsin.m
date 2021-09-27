## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_hvsin (@var{z})
## Compute the haversine function
## @end deftypefn

function res = sf_hvsin(z)
  res = sf_sin(z / 2) .^ 2;
endfunction
