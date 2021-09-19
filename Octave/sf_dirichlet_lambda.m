## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_dirichlet_lambda (@var{z})
## Compute Dirichlet lambda function $\lambda(z)$
## @end deftypefn
function res = sf_dirichlet_lambda(z)
  if (z == 1.0)
    res = Inf; # nan?
  else
    res = zeta(z) * (1.0 - 2^(-z));
  endif
endfunction
