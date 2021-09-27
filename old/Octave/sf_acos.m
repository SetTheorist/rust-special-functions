## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_acos (@var{z})
## Compute arc-cosine
## @end deftypefn

function res = sf_acos(z)
  res = -I*sf_log(z + sf_sqrt(z.^2 - 1));
endfunction
