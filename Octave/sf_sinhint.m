## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_sinhint (@var{z})
## Compute the hyperbolic sine-integral $Shi(z)$
## @end deftypefn

function res = sf_sinhint(z)
  res = (sf_expint_ei(z) + sf_expint_en(z,1))/2;
endfunction
