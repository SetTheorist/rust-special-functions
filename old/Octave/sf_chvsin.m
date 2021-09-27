## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_chvsin (@var{z})
## Compute the cohaversine/hacoversine function
## @end deftypefn

function res = sf_chvsin(z)
  res = sf_cvsin(z) / 2;
endfunction
