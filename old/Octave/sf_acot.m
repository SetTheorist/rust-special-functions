## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_acot (@var{z})
## Compute arc-cotangent
## @end deftypefn

function res = sf_acot(z)
  res = -I*0.5*sf_log((z+I)./(z-I));
endfunction
