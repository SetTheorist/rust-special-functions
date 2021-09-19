## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_hvcos (@var{z})
## Compute the havercosine function
## @end deftypefn

function res = sf_hvcos(z)
  res = sf_cos(z / 2) .^ 2;
endfunction
