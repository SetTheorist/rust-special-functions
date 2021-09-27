## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_tanh (@var{z})
## Compute hyperbolic tangent
## @end deftypefn

function res = sf_tanh(z)
  ee = sf_exp_m1(2*z);
  res = (ee) ./ (ee + 2);
endfunction
