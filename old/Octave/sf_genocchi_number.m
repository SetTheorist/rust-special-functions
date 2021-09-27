## -*- texinfo -*-
## @deftypefn {Function File} {@var{res} =} sf_genocchi_number (@var{n})
## Compute Genocchi number $G_n$
## @end deftypefn

function res = sf_genocchi_number(n)
  if (nargin < 1)
    print_usage;
  endif
  res = 2 * (1 - 2.^n) .* sf_bernoulli_number(n);
endfunction
