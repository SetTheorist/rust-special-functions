## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_atan (@var{z})
## Compute arc-tangent
## @end deftypefn

function res = sf_atan(z)
  res = -I*0.5*sf_log((1+I*z)./(1-I*z));
  if (isreal(z)) res = real(res); endif
endfunction
