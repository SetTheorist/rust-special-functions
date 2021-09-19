## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_hcvsin (@var{z})
## Compute the cohaversine/hacoversine function
## @end deftypefn

function res = sf_hcvsin(z)
  res = sf_chvsin(z);
endfunction
