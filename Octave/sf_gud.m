## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_gud (@var{z})
## Compute the Gudermannian function
## @end deftypefn

function res = sf_gud(z)
  #res = 2*sf_atan(sf_exp(z)) - pi/2;
  #res = 2*sf_atan(sf_tanh(z/2));
  #res = sf_atan(sf_sinh(z));
  res = sf_asin(sf_tanh(z));
endfunction
