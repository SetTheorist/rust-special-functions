## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_dirichlet_eta (@var{z})
## Compute Dirichlet eta function $\eta(z)$
## @end deftypefn
function res = sf_dirichlet_eta(z)
  if (z == 1.0)
    res = sf_log(2);
  else
    res = zeta(z) * (1.0 - 2^(1-z));
  endif
endfunction
