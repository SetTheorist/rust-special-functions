## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_cot (@var{z})
## Compute cotangent
## @end deftypefn

function res = sf_cot(z)
  res = sf_cos(z) ./ sf_sin(z);
endfunction
