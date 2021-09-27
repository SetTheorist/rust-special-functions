## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_coshint (@var{z})
## Compute the hyperbolic cosine-integral $Chi(z)$
## @end deftypefn

function res = sf_coshint(z)
  res = (sf_expint_ei(z) - sf_expint_en(z,1))/2;
endfunction
