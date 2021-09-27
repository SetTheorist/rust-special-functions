## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_vcos (@var{z})
## Compute the vercosine function
## @end deftypefn

function res = sf_vcos(z)
  res = 2 * sf_cos(z / 2) .^ 2;
endfunction
