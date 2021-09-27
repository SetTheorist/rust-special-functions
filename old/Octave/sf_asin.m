## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_asin (@var{z})
## Compute arc-sine
## @end deftypefn

function res = sf_asin(z)
  res = -I*sf_log(I*z + sf_sqrt(1 - z.^2));
endfunction
