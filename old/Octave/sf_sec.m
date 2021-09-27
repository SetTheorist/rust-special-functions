## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_sec (@var{z})
## Compute secant
## @end deftypefn

function res = sf_sec(z)
  res = 1 ./ sf_cos(z);
endfunction
