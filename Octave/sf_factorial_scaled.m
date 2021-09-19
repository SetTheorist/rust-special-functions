## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_factorial_scaled (@var{n})
## Compute (generalised) factorial scaled as $n! / (2\pi)^n$
## @end deftypefn

function res = sf_factorial_scaled(n)
  if (nargin<1) print_usage; endif
  res = sf_exp(sf_ln_gamma(n+1) - n*1.837877066409345483560659472811235279723);
endfunction
