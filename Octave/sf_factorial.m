## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_factorial (@var{n})
## Compute (generalised) factorial
## @end deftypefn

function res = sf_factorial(n)
  if (nargin<1)
    print_usage;
  endif
  if (n==fix(n) && imag(n)==0)
    res = round(sf_gamma(n+1));
  else
    res = sf_gamma(n+1);
  endif
endfunction
