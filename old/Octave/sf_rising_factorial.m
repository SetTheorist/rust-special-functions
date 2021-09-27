## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_rising_factorial (@var{z}, @var{alpha})
## Compute the (generalized) rising factorial $z^{\downarrow\alpha} = (z)_\al$
## @end deftypefn

function res = sf_rising_factorial(z, al)
  if (nargin<2)
    print_usage;
  endif
  res = sf_pochhammer(z, al);
endfunction
