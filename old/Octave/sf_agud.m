## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_agud (@var{z})
## Compute the inverse Gudermannian function
## @end deftypefn

function res = sf_agud(z)
  #res = sf_log(abs((1+sf_sin(z))/sf_cos(z)));
  #res = sf_log(abs((1+sf_sin(z))/(1-sf_sin(z))))/2;
  #res = sf_log(abs(sf_tan(z) + sf_sec(z));
  #res = sf_log(abs(sf_tan(pi/4 + z/2)));
  #res = sf_atanh(sf_sin(z));
  res = sf_asinh(sf_tan(z));
endfunction
