## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_ln_gamma (@var{z})
## Compute logarithm of Gamma function -- just a wrapper of built-in lgamma()
##  for real values and using Spouge's approximation (a=13) for complex arguments
## @end deftypefn

function res = sf_ln_gamma(z)
  res = zeros(size(z));
  for kk = 1:prod(size(z))
    res(kk) = sf_ln_gamma_1(z(kk));
  endfor
endfunction
function res = sf_ln_gamma_1(z)
  if (imag(z)!=0.0)
    a = 13;
    z -= 1;
    res = (z+1/2)*sf_log(z+a) - (z+a);
    sm = sqrt(2*pi);
    for k = 1 : (a-1)
      sm += spouge_c(k,a) / (z+k);
    endfor
    res += sf_log(sm);
  else
    res = lgamma(z);
  endif
endfunction
function res = spouge_c(k,a)
  res = ((-1)^(k-1) / factorial(k-1)) * (-k+a)^(k-1/2) * sf_exp(-k + a);
endfunction
