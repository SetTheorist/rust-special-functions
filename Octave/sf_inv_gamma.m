## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_inv_gamma (@var{z})
## Compute inverse (reciprocal) of Gamma function
## @end deftypefn

function res = sf_inv_gamma(z)
  res = sf_exp(-sf_ln_gamma(z));
  res(imag(z)==0.0) = real(res(imag(z)==0.0));
endfunction
