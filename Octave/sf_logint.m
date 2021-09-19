## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_logint (@var{z})
## Compute the logarithmic-integral $li(z) = \int_0^x dt/\ln(t)$
## @end deftypefn

function res = sf_logint(z)
  res = sf_expint_ei(sf_log(z));
endfunction
