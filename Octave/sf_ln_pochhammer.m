## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_ln_pochhammer (@var{z}, @var{alpha})
## Compute the logarithm of the Pochhammer symbol $\ln( (z)_\alpha )$
## @end deftypefn

function res = sf_ln_pochhammer(z, al)
  if (nargin<2)
    print_usage;
  endif
  res = sf_ln_gamma(z + al) - sf_ln_gamma(z);
endfunction
