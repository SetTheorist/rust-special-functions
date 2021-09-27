## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_ln_factorial (@var{n})
## Compute logarithm of (generalised) factorial
## @end deftypefn

function res = sf_ln_factorial(n)
  if (nargin<1)
    print_usage;
  endif
  res = sf_ln_gamma(n+1);
endfunction
