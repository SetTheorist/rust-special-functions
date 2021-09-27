## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_falling_factorial (@var{z}, @var{alpha})
## Compute the (generalized) falling factorial $z^{\downarrow\alpha}$
## @end deftypefn

function res = sf_falling_factorial(z, al)
  if (nargin<2)
    print_usage;
  endif
  res = sf_exp(sf_ln_gamma(z+1) - sf_ln_gamma(z - al + 1));
  # kludge cleanup
  if (sf_is_int(z) && sf_is_int(al)) res = round(res); endif
endfunction
