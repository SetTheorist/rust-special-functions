## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_csc (@var{z})
## Compute cosecant
## @end deftypefn

function res = sf_csc(z)
  res = 1.0 ./ sf_sin(z);
endfunction
